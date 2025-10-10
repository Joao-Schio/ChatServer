import json
import random
import socket
import string
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import List, Dict, Tuple

# =========================
# Config
# =========================
IP_SERVIDOR = "127.0.0.1"
PORTA = 8000

NUMERO_USUARIOS = 1000
NUMERO_CHAT_PRIV = 500
CHAT_PRIVADO_TAM_MIN = 2
CHAT_PRIV_TAM_MAX = 50
MES_GLOBAL_MIN = 4000
MIN_MESPRIV_POR_CHAT = 100

# concurrency + pacing
THREADS = 128
TIMEOUT = 3.0
TIMEOUT_LEITURA = 5.0
RETRIES = 3
TEMPO_SLEEP = 0 

def mandar_msg_json(mensagem: Dict) -> Tuple[bool, str]:
    msg = json.dumps(mensagem, ensure_ascii=False) + "\n"
    tentativa = 0
    while True:
        try:
            with socket.create_connection((IP_SERVIDOR, PORTA), timeout=TIMEOUT) as sock:
                sock.settimeout(TIMEOUT_LEITURA)
                sock.sendall(msg.encode("utf-8"))
                sock.shutdown(socket.SHUT_WR)
                reply = sock.recv(4096)
                return True, reply.decode(errors="replace")
        except Exception as e:
            tentativa += 1
            if tentativa >= RETRIES:
                return False, f"{type(e).__name__}: {e}"
            time.sleep(0.1 * (2 ** (tentativa - 1)))


def gerar_usuarios(n: int) -> List[str]:
    base = [f"USUARIO{i:04d}" for i in range(1, n + 1)]
    salt = "".join(random.choices(string.ascii_lowercase + string.digits, k=4))
    return [f"{u}_{salt}" for u in base]

def criar_gps(usuarios: List[str], num_grupos: int, tamam_min: int, tam_max: int) -> List[List[str]]:
    grupo = []
    for _ in range(num_grupos):
        tamanho = random.randint(tamam_min, tam_max)
        grupo.append(random.sample(usuarios, tamanho))
    return grupo

def registrar_usuarios(username: str) -> Tuple[str, bool, str]:
    msg = {"tipo": "Ola", "Usuario": username, "conteudo": username}
    ok, resposta = mandar_msg_json(msg)
    return (username, ok, resposta)

def mandar_msg_global(usuario: str, conteudo: str) -> Tuple[str, bool, str]:
    msg = {"tipo": "MensagemGlobal", "Usuario": usuario, "conteudo": conteudo}
    ok, reply = mandar_msg_json(msg)
    return (usuario, ok, reply)

def mandar_msg_privada(remetente: str, usuarios: List[str], conteudo: str) -> Tuple[str, bool, str]:
    payload = {
        "tipo": "MensagemPrivada",
        "Usuario": remetente,
        "usuarios": usuarios,
        "conteudo": conteudo,
    }
    ok, resposta = mandar_msg_json(payload)
    return (remetente, ok, resposta)

def main():
    random.seed()

    usuarios = gerar_usuarios(NUMERO_USUARIOS)
    print(f"Registrando {len(usuarios)} usuarios")
    ok = reg_err = 0
    with ThreadPoolExecutor(max_workers=THREADS) as pool:
        futures = [pool.submit(registrar_usuarios, u) for u in usuarios]
        for i, fut in enumerate(as_completed(futures), 1):
            username, ok, reply = fut.result()
            ok += ok
            reg_err += (not ok)
            if i % 200 == 0:
                print(f"  Registrado: {i}/{len(usuarios)} (ok={ok}, err={reg_err})")
            time.sleep(TEMPO_SLEEP / 1000.0)

    print(f"Fim dos cadastros ok={ok}, err={reg_err}\n")

    print(f"Criando {NUMERO_CHAT_PRIV} chats privados com tamanho {CHAT_PRIVADO_TAM_MIN}-{CHAT_PRIV_TAM_MAX}...")
    grupos = criar_gps(usuarios, NUMERO_CHAT_PRIV, CHAT_PRIVADO_TAM_MIN, CHAT_PRIV_TAM_MAX)
    print("Grupos criados.\n")

    print(f"Mandando pelo menos {MES_GLOBAL_MIN} mensagens globais")
    global_ok = global_err = 0
    with ThreadPoolExecutor(max_workers=THREADS) as pool:
        futures = []
        for i in range(MES_GLOBAL_MIN):
            remetente = random.choice(usuarios)
            conteudo = f"[global #{i+1}] Oi de {remetente}"
            futures.append(pool.submit(mandar_msg_global, remetente, conteudo))
        for i, fut in enumerate(as_completed(futures), 1):
            remetente, ok, reply = fut.result()
            global_ok += ok
            global_err += (not ok)
            if i % 500 == 0:
                print(f" Mensagem: {i}/{MES_GLOBAL_MIN} (ok={global_ok}, err={global_err})")
            time.sleep(TEMPO_SLEEP / 1000.0)

    print(f"Mensagens globais enviadas. ok={global_ok}, err={global_err}\n")

    msg_priv_totais = 0
    priv_ok = priv_err = 0
    print(f"Mandando {MIN_MESPRIV_POR_CHAT} mensagens em {len(grupos)} conversas privadas "
          f"(~{len(grupos)*MIN_MESPRIV_POR_CHAT} total)...")
    with ThreadPoolExecutor(max_workers=THREADS) as pool:
        futures = []
        for idx, members in enumerate(grupos, 1):
            msg_no_chat = MIN_MESPRIV_POR_CHAT
            msg_priv_totais += msg_no_chat
            for j in range(msg_no_chat):
                remetente = random.choice(members)
                conteudo = f"[chat privado {idx} msg {j+1}] oi de {remetente}"
                futures.append(pool.submit(mandar_msg_privada, remetente, members, conteudo))

        for i, fut in enumerate(as_completed(futures), 1):
            remetente, ok, reply = fut.result()
            priv_ok += ok
            priv_err += (not ok)
            if i % 1000 == 0:
                print(f"Msgs Privadas: {i}/{msg_priv_totais} (ok={priv_ok}, err={priv_err})")
            time.sleep(TEMPO_SLEEP / 1000.0)

    print(f"\nTodas mensagens enviadas ok={priv_ok}, err={priv_err}")
    print("\nResumo:")
    print(f"  Usuarios:            {len(usuarios)} (reg ok={ok}, err={reg_err})")
    print(f"  Msgs globais:        meta {MES_GLOBAL_MIN} (ok={global_ok}, err={global_err})")
    print(f"  Chats privados:      {len(grupos)}")
    print(f"  mensagens privadas   Meta {msg_priv_totais} (ok={priv_ok}, err={priv_err})")

if __name__ == "__main__":
    main()

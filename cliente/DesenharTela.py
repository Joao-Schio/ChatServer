import json
import os
from typing import Optional

def desenhar_tela(resposta_servidor: str, me: Optional[str] = None):
    try:
        dados = json.loads(resposta_servidor)
    except json.JSONDecodeError:
        print("Erro: resposta do servidor inválida")
        return

    os.system("cls" if os.name == "nt" else "clear")

    globais = dados.get("globais", [])
    privadas = dados.get("privadas", [])

    print("=== Conversa Global ===\n")
    if globais:
        for msg in globais:
            usuario = msg.get("usuario", "??")
            conteudo = msg.get("conteudo", "")
            print(f"{usuario}: {conteudo}")
    else:
        print("(sem mensagens globais)")

    printed_any_priv = False

    if isinstance(privadas, list) and privadas:
        is_conv_shape = any(isinstance(p, dict) and ("mensagens" in p or "integrantes" in p) for p in privadas)

        if is_conv_shape:
            for conv in privadas:
                if not isinstance(conv, dict):
                    continue
                integrantes = conv.get("integrantes", [])
                if me and integrantes and me not in integrantes:
                    continue  
                msgs = conv.get("mensagens", [])
                if not msgs:
                    continue
                if not printed_any_priv:
                    print("\n=== Mensagens Privadas ===\n")
                    printed_any_priv = True
                titulo = ", ".join(integrantes) if integrantes else "(sem integrantes)"
                print(f"[conversa privada: {titulo}]")
                for m in msgs:
                    usuario = m.get("usuario", "??")
                    conteudo = m.get("conteudo", "")
                    print(f"  {usuario}: {conteudo}")
        else:
            print("\n=== Mensagens Privadas ===\n")
            printed_any_priv = True
            for m in privadas:
                usuario = m.get("usuario", "??")
                conteudo = m.get("conteudo", "")
                print(f"[privado] {usuario}: {conteudo}")

    if not printed_any_priv:
        print("\n(sem mensagens privadas)")

    print("\n==========================")
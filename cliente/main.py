from cliente import Cliente
import json
from DesenharTela import desenhar_tela


def get_campos(entrada : str, nome_usuario : str) -> dict:
    campos = entrada.split()
    saida = {}
    if campos[0] == "/m":
        saida = {
            "usuario" : nome_usuario,
            "conteudo" : f" ".join(campos[i] for i in range(1, len(campos))),
            "tipo" : "MensagemGlobal"
        }
    if campos[0] == "/mespriv":
        saida = {
            "usuario" : nome_usuario,
            "usuarios" : campos[1].split(","),
            "tipo" : "MensagemPrivada",
            "conteudo" : f" ".join(campos[i] for i in range(2, len(campos))),
        }
    if campos[0] == "/q":
        saida = {
            "usuario": nome_usuario,
            "tipo": "Tchau",
            "conteudo": f"{nome_usuario} saiu da conversa"
        }
    if campos[0] == "/mes":
        saida = {
            "usuario": nome_usuario,
            "tipo": "GetMensagens",
            "conteudo": "nao importante"
        }
    if campos[0] == "Oi":
        saida = {
            "usuario": nome_usuario,
            "tipo": "Ola",
            "conteudo": f"{nome_usuario}"
        }

    return saida

def get_mensagem(cliente : Cliente, nome_usuario : str):
    oi = json.dumps(get_campos("Oi", nome_usuario))
    cliente.mandar_mensagem(oi)
    while True:
        try:
            entrada = input()
            mes_servidor = json.dumps(get_campos(entrada, nome_usuario))
            cliente.mandar_mensagem(mes_servidor)
            desenhar_tela(cliente.mandar_mensagem(json.dumps(get_campos("/mes", nome_usuario))))

        except:
            tchau = get_campos("/q", nome_usuario)
            mes_servidor = json.dumps(get_campos(tchau, nome_usuario))
            cliente.mandar_mensagem(mes_servidor)
            
            break


def main():
    cliente = Cliente("127.0.0.1", 8000)
    nome_usuario = input("Digite seu nome de usuario\n")
    if len(nome_usuario.split()) > 1:
        print("Nome de usuario não pode ter espaços")
        exit(1)
    get_mensagem(cliente, nome_usuario)



if __name__ == "__main__":
    main()
    




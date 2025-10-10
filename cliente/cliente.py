import socket
import json




class Cliente:
    def __init__(self, ip, porta):
        self.ip = ip
        self.porta = porta
    
    def _formatar_mensagem(mensagem_json: str) -> str:
        if mensagem_json[len(mensagem_json) - 1] != "\n":
            mensagem_json += "\n"
        
        return mensagem_json
        

    def mandar_mensagem(self, mensagem_json : str) -> str:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.connect((self.ip, self.porta))
        mensagem = Cliente._formatar_mensagem(mensagem_json)
        sock.sendall(mensagem.encode("utf-8"))
        sock.shutdown(socket.SHUT_WR)

        chunks = []
        while True:
            data = sock.recv(4096)
            if not data:
                break
            chunks.append(data)

        reply = b"".join(chunks).decode("utf-8", errors="replace")
        print(reply) # desenhar tela tira esse print
        sock.close()
        return reply

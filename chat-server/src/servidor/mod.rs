use std::io;
use std::sync::mpsc::Sender;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::Receiver;
use std::sync::Arc;
use crate::conexao::Conexao;
use crate::mensagem::Mensagem;
use crate::processador::{AcoesServidor, Processador};
use crate::usuario::Usuario;




pub struct Servidor{
    ip : String,
    porta : u16,
    usuarios : Vec<Usuario>,
    canal_mensagens : Receiver<AcoesServidor>,
    ligado : bool,
    processador : Arc<Processador>
}


impl Servidor{
    pub fn new(ip : String, porta : u16, canal : Receiver<AcoesServidor>, processador : Arc<Processador>) -> Self{
        Self{
            ip: ip,
            porta : porta,
            usuarios : Vec::new(),
            canal_mensagens : canal,
            ligado : false,
            processador : processador
        }
    }

    pub async fn get_mensagem(stream: &mut TcpStream) -> io::Result<String> {
        // Wrap a temporary BufReader around the &mut TcpStream
        let mut reader = BufReader::new(stream);
        let mut linha = String::new();

        let n = reader.read_line(&mut linha).await?;
        if n == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "peer closed"));
        }

        if linha.ends_with('\n') { linha.pop(); }
        if linha.ends_with('\r') { linha.pop(); }

        Ok(linha)
    }

    pub fn adicionar_usuario(&mut self, nome : String) -> {
        
    }

    pub async fn processar_tarefas(&mut self) -> std::io::Result<()>{
        while let Some(mes) = self.canal_mensagens.recv().await{
            match mes{
                AcoesServidor::NovoUsuario()
            }
        }
        Ok(())
    }

    pub async fn start(&mut self) -> std::io::Result<()>{
        self.ligado = true;
        let con = format!("{}:{}", &self.ip, self.porta);
        let conexao = TcpListener::bind(con).await?;
        println!("Servidor ligado em {}:{}", self.ip, self.porta);
        while self.ligado {
            let (mut stream, _) = conexao.accept().await?;
            let mensagem = Self::get_mensagem(&mut stream).await?;
            
        }
        Ok(())
    }
}
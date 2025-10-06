use tokio::sync::mpsc::{Sender, error::SendError};

use crate::mensagem::Mensagem;

pub enum AcoesServidor{
    NovoUsuario(String),
    MensagemGlobal(String, String),
    MensagemPrivada(String, String, String),
    Tchau(String)
}


pub struct Processador{
    canal_servidor : Sender<AcoesServidor>,
}


impl Processador{
    pub fn new(canal : Sender<AcoesServidor>) -> Self{
        Self{
            canal_servidor : canal
        }
    }

    pub async fn processar_mensagem(&self, mensagem : Mensagem) -> Result<String, SendError<AcoesServidor>>{
        let acao = match mensagem{
            Mensagem::Ola(nome) => {
                AcoesServidor::NovoUsuario(nome)
            },
            Mensagem::MensagemGlobal((nome, conteudo)) => {
                AcoesServidor::MensagemGlobal(nome, conteudo)
            },
            Mensagem::MensagemPrivada((usuario, destino, conteudo)) => {
                AcoesServidor::MensagemPrivada(usuario, destino, conteudo)
            },
            Mensagem::Tchau(nome) =>{
                AcoesServidor::Tchau(nome)
            }
        };
        self.canal_servidor.send(acao).await?;
        return Ok("feito".to_string());
    }
}
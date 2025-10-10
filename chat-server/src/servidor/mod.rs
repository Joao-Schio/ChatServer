use crate::{
    comando::Comando, conversa::{conversa_privada::{ConversaPrivada, ConversaPrivadaDTO}, Conversa}, mensagem::{Mensagem, MensagemDTO, MensagemJson, TipoMensagem}, resposta_servidor::RespostaServidor, usuario::{RefUsuario, Usuario}
};
use serde::Serialize;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use std::{collections::HashSet, sync::Arc};
use crate::debug_print;

pub trait ToComando{
    fn to_comando(&self, mes_json : MensagemJson) -> Result<Comando, ErroServidor>;
}

pub trait ProcessarComando{
    fn processar_comando(&mut self, comando : Comando) -> Result<RespostaServidor, ErroServidor>;
}    


#[derive(Clone, Copy, Serialize)]
pub enum ErroServidor{
    UsuarioExistente,
    UsuarioNaoExistente,
    JsonInvalido,
    FalhaAoEscrever,
    FalhaAoLer,
}

impl From<ErroServidor> for std::io::Error {
    fn from(e: ErroServidor) -> Self {
        use ErroServidor::*;
        let msg = match e {
            UsuarioExistente => "usuario já existe",
            UsuarioNaoExistente => "usuario não existe",
            JsonInvalido => "Json invalido",
            FalhaAoEscrever => "Falha ao escrever",
            FalhaAoLer => "Falha ao ler"
        };
        std::io::Error::new(std::io::ErrorKind::Other, msg)
    }
}


pub struct Servidor{
    ip : String,
    porta : u16,
    usuarios : Vec<RefUsuario>,
    mensagens_globais : Conversa,
    mensagens_privadas : Vec<ConversaPrivada>,
    ligado : bool
}


impl Servidor {
    pub fn new(ip : String, porta : u16) -> Self{
        Self{
            ip : ip,
            porta : porta,
            usuarios : Vec::new(),
            mensagens_globais : Conversa::new(),
            mensagens_privadas : Vec::new(),
            ligado : false
        }
    }

    async fn handle_error(&self,erro: ErroServidor,stream: &mut TcpStream) {
        let resposta = RespostaServidor::Erro { erro };
        let json = serde_json::to_string(&resposta)
            .unwrap_or_else(|_| r#"{"tipo":"erro","erro":"Interno"}"#.to_string());
        let _ = stream.write_all(json.as_bytes()).await;
        let _ = stream.write_all(b"\n").await;
    }


    fn criar_usuario(&self, nome : String) -> Result<RefUsuario, ErroServidor>{
        let existe = self.usuarios.iter().any(
            |x| x.get_nome() == &nome
        );

        if existe{
            return Err(ErroServidor::UsuarioExistente)
        }

        let usuario = Arc::new(
            Usuario::new(nome)
        );

        Ok(usuario)
    }

    

    pub fn get_usuario(&self, nome : &String) -> Result<RefUsuario, ErroServidor>{
        let user = self.usuarios.iter()
                                          .find(|x| x.get_nome() == nome);
        match user{
            Some(usuario) => {
                Ok(usuario.clone())
            },
            None => {
                Err(ErroServidor::UsuarioNaoExistente)
            }
        }
    }

    pub fn get_usuario_mut(&mut self, nome : &String) -> Result<&mut RefUsuario, ErroServidor>{
        let user = self.usuarios.iter_mut()
                                          .find(|x| x.get_nome() == nome);
        match user{
            Some(usuario) =>{
                Ok(usuario)
            },
            None => {
                Err(ErroServidor::UsuarioNaoExistente)
            }
        }

    }

    pub async fn get_mensagem(stream : &mut TcpStream) -> Result<MensagemJson, ErroServidor>{
        let mut buffer_mensagens = Vec::new();
        let _ = stream.read_to_end(&mut buffer_mensagens).await.map_err(|_| ErroServidor::FalhaAoLer)?;
        let json : MensagemJson = serde_json::from_slice(&buffer_mensagens)
                                  .map_err(|_| ErroServidor::JsonInvalido)?;
    
        debug_print!("json", &json);
        
    
        Ok(json)
    }

    pub async fn iniciar_servidor(&mut self) -> tokio::io::Result<()>{
        self.ligado = true;
        let endereco = format!("{}:{}", &self.ip, self.porta);
        println!("iniciando o servidor em {}", &endereco);
        let listener = TcpListener::bind(endereco).await?;
        
        while self.ligado{
            let (mut stream, _end_cliente) = listener.accept().await?;
            
            match self.processar_stream(&mut stream).await {
                Ok(()) => {()},
                Err(e) => {
                    self.handle_error(e, &mut stream).await;
                    continue;
                }
            }
            debug_print!("nova conexao ", &_end_cliente);
                        
        }

        Ok(())
    }


    pub async fn processar_stream(&mut self, stream: &mut TcpStream) -> Result<(), ErroServidor>{
        let mes = Self::get_mensagem(stream).await?;
        let comando = self.to_comando(mes)?;
        let resposta: RespostaServidor = self.processar_comando(comando)?;
        
        let json_resposta = serde_json::to_string(&resposta).map_err(|_| ErroServidor::JsonInvalido)?;
        stream.write(json_resposta.as_bytes()).await.map_err(|_| ErroServidor::FalhaAoEscrever)?;
        debug_print!("Mensagens globais ", &self.mensagens_globais);
        debug_print!("Usuarios online ", &self.usuarios);
        Ok(())
    }
}


impl ToComando for Servidor{
    fn to_comando(&self, mes_json : MensagemJson) -> Result<Comando, ErroServidor> {
        use TipoMensagem::*;
        use ErroServidor::*;
        match mes_json.tipo {
            Ola(nome) if self.get_usuario(&nome).is_ok() => Err(UsuarioExistente),
            Ola(nome) => Ok(Comando::AdicionarUsuario { nome }),
            MensagemGlobal(conteudo) => {
                let usuario = self.get_usuario(&mes_json.usuario)?;
                Ok(Comando::MensagemGlobal { usuario: usuario, conteudo: conteudo })
            },
            Tchau(_) => {
                let usuario =  self.get_usuario(&mes_json.usuario)?;
                Ok(Comando::Tchau { usuario: usuario })
            },
            MensagemPrivada(conteudo) => {
                let nomes= mes_json.usuarios.ok_or(UsuarioNaoExistente)?;
                let remetente = self.get_usuario(&mes_json.usuario)?;
                
                let mut integrantes: Vec<Arc<Usuario>> = nomes.iter().map(|nome| 
                    self.usuarios.iter()
                    .find(|u| u.get_nome() == nome)
                    .cloned()
                    .ok_or(UsuarioNaoExistente)
                ).collect::<Result<_, _>>()?;

                integrantes.push(remetente.clone());

                Ok(
                    Comando::MensagemPrivada { integrantes: integrantes, remetente : remetente, mensagem: conteudo }
                )
            },
            GetMensagens(_) => {
                let usuario =  self.get_usuario(&mes_json.usuario)?;
                Ok(
                    Comando::GetMensagems { usuario: usuario }
                )
            }
        }
    }
}


impl ProcessarComando for Servidor{
    fn processar_comando(&mut self, comando : Comando) -> Result<RespostaServidor, ErroServidor> {
        use Comando::*;
        match comando{
            MensagemGlobal { usuario, conteudo } => {
                let mensagem = Mensagem::new(usuario, conteudo);
                self.mensagens_globais.adicionar_mensagem(mensagem);
                Ok(
                    RespostaServidor::MensagemAdicionada
                )
            },
            AdicionarUsuario { nome } => {
                let usuario = self.criar_usuario(nome)?;
                self.usuarios.push(usuario);
                Ok(
                    RespostaServidor::UsuarioAdicionado
                )
            },
            Tchau { usuario } => {
                usuario.set_online(false);
                Ok(RespostaServidor::UsuarioOffline)
            },
            MensagemPrivada { integrantes, remetente, mensagem } => {
            
                let usuarios_set: HashSet<_> = integrantes.iter().cloned().collect();
                
                let conv = self.mensagens_privadas.iter_mut().find(|conversa| {
                    let integrantes_set: HashSet<_> = conversa.get_usuarios().iter().cloned().collect();
                    integrantes_set == usuarios_set
                });

                let msg = Mensagem::new(remetente, mensagem);

                match conv{
                    Some(c) => {c.get_conversa_mutavel().adicionar_mensagem(msg);},
                    None => {
                        let mut nova = ConversaPrivada::from_vetor(integrantes);
                        nova.get_conversa_mutavel().adicionar_mensagem(msg);
                        self.mensagens_privadas.push(nova);
                    }
                }

                Ok(
                    RespostaServidor::MensagemAdicionada
                )
            },
            GetMensagems { usuario } => {
                let globais = self.mensagens_globais.get_mensagens().iter().map(|x| x.into()).collect::<Vec<MensagemDTO>>();
                let privadas = self.mensagens_privadas.iter().filter(
                    |x| x.get_usuarios().contains(&usuario)
                ).map(|x| x.into()).collect::<Vec<ConversaPrivadaDTO>>();
                Ok(
                    RespostaServidor::GetMensagens{globais : globais, privadas : privadas}
                )
            }
        }

    }
}


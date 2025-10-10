use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::usuario::RefUsuario;


#[derive(Deserialize, Debug)]
#[serde(tag = "tipo", content = "conteudo")]
pub enum TipoMensagem{
    #[serde(rename = "Ola")]
    Ola(String),
    #[serde(rename = "Tchau")]
    Tchau(String),
    #[serde(rename = "MensagemGlobal")]
    MensagemGlobal(String),
    #[serde(rename = "MensagemPrivada")]
    MensagemPrivada(String),
    GetMensagens(String)
}

#[derive(Debug)]
pub struct Mensagem{
    usuario : RefUsuario,
    conteudo : String,
    hora_envio : std::time::SystemTime,
}

impl Mensagem{
    pub fn new(usuario : RefUsuario, conteudo : String) -> Self{
        Self { usuario: usuario, conteudo: conteudo, hora_envio: SystemTime::now() }
    }

    #[inline]
    pub fn get_usuario(&self) -> RefUsuario{
        self.usuario.clone()
    }

    #[inline]
    pub fn get_conteudo(&self) -> &String{
        &self.conteudo
    }

    #[inline]
    pub fn get_hora(&self) -> &SystemTime{
        &self.hora_envio
    }
}

#[derive(Deserialize, Debug)]
pub struct MensagemJson{
    #[serde(alias = "Usuario", alias = "usuario")]
    pub usuario : String,

    #[serde(flatten)]
    pub tipo : TipoMensagem,
    
    #[serde(default)]
    pub usuarios : Option<Vec<String>>
}

#[derive(Serialize, Debug)]
pub struct MensagemDTO {
    pub usuario: String,
    pub conteudo: String,
}

impl From<&Mensagem> for MensagemDTO {
    fn from(m: &Mensagem) -> Self {
        Self {
            usuario: m.usuario.get_nome().clone(),
            conteudo: m.conteudo.clone(),
        }
    }
}
use crate::{conversa::Conversa, mensagem::MensagemDTO, usuario::Usuario};
use serde::Serialize;
use std::sync::Arc;


pub struct ConversaPrivada{
    usuarios : Vec<Arc<Usuario>>,
    mensagens : Conversa
}

impl ConversaPrivada{
    pub fn new() -> Self{
        Self{
            usuarios : Vec::new(),
            mensagens : Conversa::new()
        }
    }
    #[inline]
    pub fn from_vetor(usuarios : Vec<Arc<Usuario>>) -> Self{
        Self{
            usuarios: usuarios,
            mensagens : Conversa::new()
        }
    }
    #[inline]
    pub fn adicionar_usuario(&mut self, usuario : Arc<Usuario>){
        self.usuarios.push(usuario);
    }
    #[inline]
    pub fn get_usuarios(&self) -> &Vec<Arc<Usuario>>{
        &self.usuarios
    }
    #[inline]
    pub fn get_conversa_mutavel(&mut self) -> &mut Conversa{
        return &mut self.mensagens;
    }
    #[inline]
    pub fn get_conversa(&self) -> &Conversa{
        &self.mensagens
    }

}


#[derive(Serialize)]
pub struct ConversaPrivadaDTO {
    pub integrantes: Vec<String>,
    pub mensagens: Vec<MensagemDTO>
}

impl From<&ConversaPrivada> for ConversaPrivadaDTO {
    fn from(c: &ConversaPrivada) -> Self {
        let integrantes = c
            .get_usuarios()
            .iter()
            .map(|u| u.get_nome().clone())
            .collect();
        let mensagens = c
        .get_conversa()
        .mensagens.iter().map(|x|x.into()).collect::<Vec<MensagemDTO>>();
        Self { integrantes, mensagens }
    }
}
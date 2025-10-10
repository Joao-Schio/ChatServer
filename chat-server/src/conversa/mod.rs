use crate::mensagem::Mensagem;
pub mod conversa_privada;



#[derive(Debug)]
pub struct Conversa{
    mensagens : Vec<Mensagem>
}


impl Conversa {
    pub fn new() -> Self{
        Self{
            mensagens: Vec::new()
        }
    }

    #[inline]
    pub fn adicionar_mensagem(&mut self, mensagem : Mensagem){
        self.mensagens.push(mensagem);
    }

    #[inline]
    pub fn get_mensagens(&self) -> &Vec<Mensagem>{
        &self.mensagens
    }
}
use std::collections::VecDeque;

use crate::conexao::Conexao;
use crate::mensagem::Mensagem;




pub struct Usuario{
    nome    : String,
    conexao : Conexao, 
}


impl Usuario{
    pub fn new(nome : String, conexao : Conexao) -> Self{
        Self { nome: nome, conexao: conexao }
    }

    pub async fn mandar_mensagem(&mut self, mensagens : VecDeque<Mensagem>) -> std::io::Result<()>{
        for i in &mensagens{
            self.conexao.mandar_mensagem_json(i).await?;
        }
        Ok(())
    }

    pub fn get_nome(&self) -> &String{
        return &self.nome;
    }
}
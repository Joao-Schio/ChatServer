use crate::usuario::RefUsuario;



pub enum Comando{
    AdicionarUsuario{nome: String},
    MensagemGlobal{usuario: RefUsuario, conteudo: String},
    Tchau{usuario: RefUsuario},
    MensagemPrivada {integrantes : Vec<RefUsuario>, remetente : RefUsuario, mensagem : String},
    GetMensagems{usuario : RefUsuario}
}
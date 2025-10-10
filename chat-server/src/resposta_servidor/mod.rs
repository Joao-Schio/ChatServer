use serde::Serialize;

use crate::{conversa::conversa_privada::ConversaPrivadaDTO, mensagem:: MensagemDTO, servidor::ErroServidor};



#[derive(Serialize)]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum RespostaServidor{
    MensagemAdicionada,
    UsuarioAdicionado,
    UsuarioOffline,
    Erro{erro : ErroServidor},
    GetMensagens{globais : Vec<MensagemDTO>, privadas : Vec<ConversaPrivadaDTO>}
}
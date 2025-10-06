type MensagemPrivada = (String, String, String);
type MensagemGlobal  = (String, String);


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Mensagem{
    Ola(String),
    Tchau(String),
    MensagemGlobal(MensagemGlobal),
    MensagemPrivada(MensagemPrivada),
}
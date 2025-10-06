use tokio::sync::mpsc::channel;
use std::sync::Arc;
use chat_server::{processador::{AcoesServidor, Processador}, servidor::Servidor};


#[tokio::main]
async fn main() -> std::io::Result<()>{

    // Iniciando a injecao de dependencias

    let (sender, receiver) = channel(256);

    let processador = Arc::new(Processador::new(sender));
    let mut servidor = Servidor::new("127.0.0.1".to_string(), 5000, receiver);



    servidor.start().await?;

    Ok(())
}

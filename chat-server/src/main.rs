use chat_server::servidor::Servidor;


#[tokio::main]
async fn main() -> std::io::Result<()>{

    let mut server = Servidor::new("127.0.0.1".to_string(), 8000);

    server.iniciar_servidor().await?;

    Ok(())
}

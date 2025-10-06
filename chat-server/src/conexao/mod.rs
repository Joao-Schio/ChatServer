use std::io;
use std::sync::mpsc::Sender;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use serde::{self, Deserialize, Serialize};

use crate::mensagem::Mensagem;

pub struct Conexao {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
    canal_mensagens: Sender<Mensagem>,
}

impl Conexao {
    pub fn new(stream: TcpStream, canal_mensagens: Sender<Mensagem>) -> Self {
        let (r, w) = stream.into_split(); // <-- no try_clone; split into halves
        Self {
            reader: BufReader::new(r),
            writer: w,
            canal_mensagens,
        }
    }

    /// Send TipoMensagem as NDJSON (JSON + '\n')
    pub async fn mandar_mensagem_json(&mut self, mensagem: &Mensagem) -> io::Result<()> {
        let mut json = serde_json::to_vec(mensagem).map_err(to_io_err)?;
        json.push(b'\n'); // frame boundary
        self.writer.write_all(&json).await?;
        self.writer.flush().await?;
        Ok(())
    }

    /// Receive NDJSON from socket and forward via channel
    pub async fn receber_mensagens(&mut self) -> io::Result<()> {
        let mut linha = String::new();
        loop {
            linha.clear();
            let n = self.reader.read_line(&mut linha).await?;
            if n == 0 {
                break; // peer closed
            }
            if linha.ends_with('\n') { linha.pop(); }
            if linha.ends_with('\r') { linha.pop(); }

            match serde_json::from_str::<Mensagem>(&linha) {
                Ok(msg) => { let _ = self.canal_mensagens.send(msg); }
                Err(e) => eprintln!("JSON parse error: {e}; raw={linha:?}"),
            }
        }
        Ok(())
    }
}

fn to_io_err<E: std::error::Error + Send + Sync + 'static>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}

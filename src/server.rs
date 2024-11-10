use crate::{
    errors::SmtpErrorCode, smtp::HandleCurrentState, CLOSING_CONNECTION, INITIAL_GREETING, TIMEOUT,
};
use std::error::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::timeout,
};

pub struct Server {
    connection: tokio::net::TcpStream,
    state_handler: HandleCurrentState,
}

impl Server {
    pub async fn new(
        server_domain: impl AsRef<str>,
        connection: tokio::net::TcpStream,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            connection,
            state_handler: HandleCurrentState::new(server_domain),
        })
    }

    pub async fn connection(mut self) -> Result<(), Box<dyn Error>> {
        self.connection.write_all(INITIAL_GREETING).await?;
        let mut buffer: Vec<u8> = vec![0; 65536];
        loop {
            match timeout(TIMEOUT, self.connection.read(&mut buffer)).await {
                Ok(Ok(0)) => {
                    break;
                }
                Ok(Ok(bytes)) => {
                    let message: &str = match std::str::from_utf8(&buffer[0..bytes]) {
                        Ok(a) => a,
                        Err(e) => {
                            return Err(Box::new(e));
                        }
                    };
                    match self.state_handler.process_smtp_command(message) {
                        Ok(response) => {
                            if response != b"" {
                                self.connection.write_all(response).await?;
                            }
                            if response == CLOSING_CONNECTION {
                                break;
                            }
                        }
                        Err(e) => {
                            self.connection
                                .write_all(e.format_response().as_bytes())
                                .await?;
                            if e.code.as_code() >= SmtpErrorCode::SyntaxError.into() {
                                break;
                            }
                        }
                    };
                }
                Ok(Err(_)) => {
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        }
        Ok(())
    }

    pub async fn serve(mut self) -> Result<(), Box<dyn Error>> {
        self.connection.write_all(INITIAL_GREETING).await?;
        let mut buf: Vec<u8> = vec![0; 1024 * 1024];
        loop {
            let n: usize = self.connection.read(&mut buf).await?;
            if n == 0 {
                self.state_handler.process_smtp_command("quit").ok();
                break;
            }
            let msg: &str = std::str::from_utf8(&buf[..n])?;
            let response: &[u8] = self.state_handler.process_smtp_command(msg)?;

        }
        todo!()
    }
}

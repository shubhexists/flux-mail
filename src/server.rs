use crate::{
    database::DatabaseClient, errors::SmtpErrorCode, smtp::HandleCurrentState, CLOSING_CONNECTION,
    INITIAL_GREETING, TIMEOUT,
};
use std::{error::Error, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::timeout,
};
use tracing::{span::Entered, Level, Span};

pub struct Server<'a> {
    connection: tokio::net::TcpStream,
    state_handler: HandleCurrentState,
    db: &'a Arc<DatabaseClient>,
}

impl<'a> Server<'a> {
    pub async fn new(
        server_domain: impl AsRef<str>,
        connection: tokio::net::TcpStream,
        db: &'a Arc<DatabaseClient>,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            connection,
            state_handler: HandleCurrentState::new(server_domain),
            db,
        })
    }

    pub async fn connection(mut self) -> Result<(), Box<dyn Error>> {
        let span: Span = tracing::span!(Level::INFO, "MAIL");
        let _enter: Entered<'_> = span.enter();

        self.connection.write_all(INITIAL_GREETING).await?;
        tracing::info!("Greeted");

        let mut buffer: Vec<u8> = vec![0; 65536];
        loop {
            match timeout(TIMEOUT, self.connection.read(&mut buffer)).await {
                Ok(Ok(0)) => {
                    tracing::error!("Unexpected End of Stream without any data.");
                    break;
                }
                Ok(Ok(bytes)) => {
                    let message: &str = match std::str::from_utf8(&buffer[0..bytes]) {
                        Ok(a) => a,
                        Err(e) => {
                            tracing::error!("Broken pipe, closing stream: {}", e);
                            return Err(Box::new(e));
                        }
                    };
                    match self
                        .state_handler
                        .process_smtp_command(message, &self.db)
                        .await
                    {
                        Ok(response) => {
                            if response != b"" {
                                self.connection.write_all(response).await?;
                            }
                            if response == CLOSING_CONNECTION {
                                tracing::warn!("Closing connection!");
                                break;
                            }
                        }
                        Err(e) => {
                            self.connection
                                .write_all(e.format_response().as_bytes())
                                .await?;
                            tracing::error!("Unexpected End of Stream, closing connection");
                            if e.code.as_code() >= SmtpErrorCode::SyntaxError.into() {
                                break;
                            }
                        }
                    };
                }
                Ok(Err(_)) => {
                    tracing::error!("Broken pipe, couldn't read stream");
                    break;
                }
                Err(_) => {
                    tracing::error!("Timeout Error: No data for 30 seconds. Closing!");
                    break;
                }
            }
        }
        Ok(())
    }
}

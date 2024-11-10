mod errors;
pub mod server;
mod smtp;
mod types;
use server::Server;
use std::time::Duration;
use std::{error::Error, net::SocketAddr};
use tokio::net::TcpListener;
use tokio::time::timeout;

const MAX_EMAIL_SIZE: usize = 10_485_760;
const TIMEOUT: Duration = Duration::from_secs(30);
const MAX_RECIPIENT_COUNT: usize = 100;
const INITIAL_GREETING: &'static [u8] = b"220 ESMTP Service Ready\n";
const SUCCESS_RESPONSE: &'static [u8] = b"250 Ok\n";
const DATA_READY_PROMPT: &'static [u8] = b"354 End data with <CR><LF>.<CR><LF>\n";
const CLOSING_CONNECTION: &'static [u8] = b"221 Goodbye\n";

pub(crate) fn is_valid_email(email: &str) -> bool {
    email.contains('@') && !email.contains("..") && email.len() < 254
}

#[tokio::main]
pub async fn start_server(addr: SocketAddr, domain: String) -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let listener: TcpListener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        tokio::task::LocalSet::new()
            .run_until(async move {
                let smtp = Server::new(&domain, stream).await?;
                match timeout(std::time::Duration::from_secs(300), smtp.serve()).await {
                    Ok(Ok(_)) => {
                        Ok(())
                    }
                    Ok(Err(e)) => {
                        // Handle the error returned from smtp.serve()
                        Err(Box::new(e) as Box<dyn Error>)
                    }
                    Err(_) => {
                        // Handle timeout
                        Err("Connection timed out".into())
                    }
                }
            })
            .await?;
    }
}

mod errors;
pub mod server;
mod smtp;
mod types;
use server::Server;
use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, net::SocketAddr};
use tokio::net::TcpListener;
use tokio::time::timeout;

const MAX_EMAIL_SIZE: usize = 10_485_760;
const TIMEOUT: Duration = Duration::from_secs(30);
const MAX_RECIPIENT_COUNT: usize = 100;
const INITIAL_GREETING: &'static [u8] = b"220 Flux Mail Service Ready\n";
const SUCCESS_RESPONSE: &'static [u8] = b"250 Ok\n";
const DATA_READY_PROMPT: &'static [u8] = b"354 End data with <CR><LF>.<CR><LF>\n";
const CLOSING_CONNECTION: &'static [u8] = b"221 Goodbye\n";
const AUTH_OK: &'static [u8] = b"235 Ok\n";

pub(crate) fn is_valid_email(email: &str) -> bool {
    email.contains('@') && !email.contains("..") && email.len() < 254
}

#[tokio::main]
pub async fn start_server(addr: SocketAddr, domain: String) -> Result<(), Box<dyn Error>> {
    let listener: TcpListener = TcpListener::bind(&addr).await?;
    let domain: Arc<String> = Arc::new(domain);

    loop {
        let (stream, _addr) = listener.accept().await?;
        let domain: Arc<String> = Arc::clone(&domain);

        tokio::task::LocalSet::new()
            .run_until(async move {
                let smtp: Server = Server::new(domain.as_str(), stream).await?;
                match timeout(Duration::from_secs(300), smtp.connection()).await {
                    Ok(Ok(_)) => Ok(()),
                    Ok(Err(e)) => Err(e),
                    Err(e) => Err(Box::new(e) as Box<dyn Error>),
                }
            })
            .await
            .ok();
    }
}

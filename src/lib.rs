mod database;
mod errors;
pub mod server;
mod smtp;
mod types;
use database::DatabaseClient;
use server::Server;
use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
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
    // let db: Arc<DatabaseClient> = Arc::new(DatabaseClient::connect().await?);
    tracing::info!("Server Started On Port: {}", addr);

    loop {
        let (stream, _addr): (TcpStream, SocketAddr) = listener.accept().await?;
        let domain: Arc<String> = Arc::clone(&domain);
        // let db: Arc<DatabaseClient> = Arc::clone(&db);

        tokio::task::LocalSet::new()
            .run_until(async move {
                tracing::info!("Ping received on SMTP Server");
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

pub fn clear_old_mails(period: tokio::time::Duration) {
    std::thread::spawn(move || -> Result<(), Box<dyn Error + Send + Sync>> {
        let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .enable_io()
            .build()
            .map_err(|e: std::io::Error| format!("Failed to build async runtime: {}", e))?;

        runtime.block_on(async move {
            let local: tokio::task::LocalSet = tokio::task::LocalSet::new();
            local.spawn_local(async move {
                let db: DatabaseClient = match DatabaseClient::connect().await {
                    Ok(db) => db,
                    Err(e) => {
                        tracing::error!("Failed to connect to database: {}", e);
                        return;
                    }
                };
                let mut interval: tokio::time::Interval = tokio::time::interval(period);
                interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
                loop {
                    interval.tick().await;
                    if let Err(e) = db.delete_old_mail().await {
                        tracing::error!("Failed to delete old mail: {}", e);
                    }
                }
            });
            local.await;
        });
        Ok(())
    });
}

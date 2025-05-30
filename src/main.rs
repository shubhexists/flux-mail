use dotenv::dotenv;
use flux_mail::{clear_old_mails, start_server};
use tokio::time;
use tracing::{error, info};

fn main() {
    tracing_subscriber::fmt::init();
    if dotenv().is_err() {
        error!("Warning: Failed to load .env file. Default environment variables may be missing.");
    } else {
        info!("Info: .env file successfully loaded.");
    }

    let addr: std::net::SocketAddr = "0.0.0.0:25".parse().unwrap();
    let domain: String = String::from("mail.flux.shubh.sh");

    clear_old_mails(time::Duration::from_secs(3600));
    if let Err(e) = start_server(addr, domain) {
        tracing::error!("Error starting server: {}", e);
        eprintln!("Error starting server: {}", e);
    }
}

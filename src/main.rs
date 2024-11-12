use flux_mail::start_server;

fn main() {
    let addr: std::net::SocketAddr = "0.0.0.0:25".parse().unwrap();
    let domain: String = String::from("mail.flux.shubh.sh");

    tracing_subscriber::fmt::init();

    if let Err(e) = start_server(addr, domain) {
        tracing::error!("Error starting server: {}", e);
        eprintln!("Error starting server: {}", e);
    }
}

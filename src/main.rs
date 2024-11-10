use flux_mail::start_server;

fn main() {
    let addr: std::net::SocketAddr = "0.0.0.0:2525".parse().unwrap();
    // ADD CHECK FOR DOMAIN
    let domain: String = String::from("example.com");
    if let Err(e) = start_server(addr, domain) {
        eprintln!("Error starting server: {}", e);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("cerebri=info")
        .init();
    let address: std::net::SocketAddr = std::env::var("CEREBRI_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".into())
        .parse()?;
    if !address.ip().is_loopback() {
        return Err("development API requires loopback".into());
    }
    let listener = tokio::net::TcpListener::bind(address).await?;
    println!("CEREBRI_LISTEN_ADDR={}", listener.local_addr()?);
    axum::serve(listener, cerebri_api::router()).await?;
    Ok(())
}

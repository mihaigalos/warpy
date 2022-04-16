use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let footer = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    warpy::server::run(".".to_string(), [0, 0, 0, 0], footer, None, true).await?;
    Ok(())
}

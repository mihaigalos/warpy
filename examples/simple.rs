use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    warpy::server::run(".".to_string(), [127, 0, 0, 1], 8082).await?;
    Ok(())
}

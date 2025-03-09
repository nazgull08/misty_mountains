use async_nats::{Client, ConnectOptions};
use std::time::Duration;
use tokio::time::sleep;

pub async fn connect_nats() -> Result<Client, Box<dyn std::error::Error>> {
    let nats_url = "nats://localhost:4222"; // Можно вынести в .env

    let client = async_nats::connect(nats_url).await?;
    
    println!("✅ Connected to NATS: {}", nats_url);

    Ok(client)
}

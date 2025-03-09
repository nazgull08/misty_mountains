use async_nats::Client;
use dotenv::dotenv;
use nats_client::connect_nats;
use sqlx::PgPool;
use tokio::sync::Mutex;
use std::env;
use std::sync::Arc;
use rocket::tokio::signal;
use rocket::tokio::sync::broadcast;

mod config;
mod crypto;
mod nats_client;
mod types;
mod web;

#[rocket::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPool::connect(&database_url).await {
        Ok(pool) => {
            println!("Connected to database!");
            pool
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e);
            return;
        }
    };

    let port = 8000;
    let (shutdown_tx, _) = broadcast::channel(1);


    let nats_client = match connect_nats().await {
        Ok(client) => Arc::new(Mutex::new(client)),
        Err(e) => {
            eprintln!("❌ Failed to connect NATS: {:?}", e);
            return;
        }
    };

    let rocket_task = spawn_rocket_server(port, pool, nats_client, shutdown_tx.subscribe());

    signal::ctrl_c().await.expect("failed to listen for Ctrl+C");
    println!("Ctrl+C received! Initiating shutdown...");

    drop(shutdown_tx);

    if let Err(e) = rocket_task.await {
        eprintln!("Rocket server error: {:?}", e);
    }

    println!("Application has shut down gracefully.");
}

fn spawn_rocket_server(
    port: u16,
    pool: PgPool,
    nats: Arc<Mutex<Client>>,
    mut shutdown: broadcast::Receiver<()>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        println!("Starting Rocket server on port {}", port);
        let rocket = web::server::rocket(port, pool, nats).await;

        tokio::select! {
            result = rocket.launch() => {
                if let Err(e) = result {
                    eprintln!("Error launching Rocket server: {:?}", e);
                }
            }
            _ = shutdown.recv() => {
                println!("Shutdown signal received. Stopping Rocket server...");
            }
        }
    })
}

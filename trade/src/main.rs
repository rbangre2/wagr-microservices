use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();
    let mongo_connection_uri = std::env::var("DATABASE_URI").expect("DATABASE_URI must be set.");
    let client_options = ClientOptions::parse(mongo_connection_uri).await?;
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("pinged your deployment. successfully connected to MongoDB!");
    Ok(())
}

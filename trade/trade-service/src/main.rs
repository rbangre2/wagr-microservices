
use mongodb::{options::ClientOptions, Client};
use dotenv::dotenv;
use std::env;
use bson::doc;

#[tokio::main]
async fn main() {
    dotenv().ok(); 

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI is not set in .env file");

    // Parse your MongoDB Atlas connection string into client options
    let client_options = ClientOptions::parse(&mongodb_uri)
        .await
        .expect("Failed to parse MongoDB URI");

    let client = Client::with_options(client_options)
        .expect("Failed to initialize MongoDB client");

    // Optionally, ping the server to see if you can connect to the cluster
    match client.database("admin").run_command(doc! {"ping": 1}, None).await {
        Ok(_) => println!("Successfully connected to MongoDB Atlas!"),
        Err(e) => eprintln!("Could not connect to MongoDB Atlas: {}", e),
    }

    // Proceed with setting up your service
}

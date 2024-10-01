use mongodb::{Client, options::ClientOptions};
use serde::{Deserialize, Serialize};
use futures::stream::TryStreamExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct UserOp {
    sender: String,
    nonce: String,
    initCode: String,
    callData: String,
    signature: String,
    paymasterAndData: String,
    maxFeePerGas: String,
    maxPriorityFeePerGas: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserOpSponsorshipRequest {
    #[serde(rename = "_id")]
    id: String,
    apiKey: String,
    requestId: String,
    chainId: i32,
    smartAccountAddress: String,
    userOp: UserOp,
    entryPointAddress: String,
    state: String,
    createdAt: i64,
    updatedAt: i64,
    #[serde(rename = "__v")]
    v: i32,
    mode: Option<String>,
    userOpHash: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the .env file
    dotenv().ok();

    // Get the MongoDB URI from the environment variable
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env file");
    
    // Parse a connection string into an options struct
    let mut client_options = ClientOptions::parse(&uri).await?;
    
    // Manually set an option
    client_options.app_name = Some("Rust MongoDB App".to_string());
    
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    
    // Get a handle to the database and collection
    let db = client.database("paymaster-dashboard-v2");
    let collection = db.collection::<UserOpSponsorshipRequest>("useropsponsorshiprequests");

    // Get the current timestamp
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;

    println!("Monitoring for new documents...");

    loop {
        // Find documents created after start_time
        let filter = doc! { "createdAt": { "$gt": start_time } };
        let mut cursor = collection.find(filter, None).await?;

        // Iterate through the results
        while let Some(doc) = cursor.try_next().await? {
            let created_at = DateTime::<Utc>::from_utc(
                chrono::NaiveDateTime::from_timestamp_millis(doc.createdAt).unwrap(),
                Utc,
            );
            println!("New document found:");
            println!("ID: {}", doc.id);
            println!("Request ID: {}", doc.requestId);
            println!("Chain ID: {}", doc.chainId);
            println!("Smart Account Address: {}", doc.smartAccountAddress);
            println!("State: {}", doc.state);
            println!("Created At: {}", created_at);
            println!("Mode: {:?}", doc.mode);
            println!("User Op Hash: {:?}", doc.userOpHash);
            println!("----------------------------------------");

            // Update start_time to the most recent document's createdAt
            start_time = doc.createdAt;
        }

        // Wait for a short duration before checking again
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
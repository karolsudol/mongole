use mongodb::{Client, options::ClientOptions, bson::doc};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use std::env;
use futures::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
struct UserOp {
    sender: String,
    nonce: String,
    init_code: String,
    call_data: String,
    signature: String,
    paymaster_and_data: String,
    max_fee_per_gas: String,
    max_priority_fee_per_gas: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PaymasterUserOperations {
    #[serde(rename = "_id")]
    id: String,
    actual_gas_cost: i64,
    actual_gas_used: i64,
    api_key: String,
    request_id: String,
    chain_id: i32,
    entry_point_address: String,
    exchange_rate: String,
    mode: String,
    paymaster_and_data: String,
    nonce: i64,
    oracle_aggregator: String,
    paymaster_address: String,
    paymaster_id: String,
    paymaster_type: String,
    paymaster_version: String,
    price_markup: String,
    price_source: bool,
    smart_account_address: String,
    state: String,
    success: bool,
    token: String,
    token_decimal: i32,
    token_symbol: String,
    transaction_hash: String,
    user_op: UserOp,
    user_op_hash: String,
    value: String,
    value_in_usd: String,
    created_at_timestamp: i64,
    created_at: i64,
    updated_at: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in environment");
    
    let mut client_options = ClientOptions::parse(&uri).await?;
    
    client_options.app_name = Some("Rust MongoDB App".to_string());
    
    let client = Client::with_options(client_options)?;
    
    let db_name = env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE must be set in environment");
    let collection_name = env::var("MONGODB_COLLECTION").expect("MONGODB_COLLECTION must be set in environment");

    let db = client.database(&db_name);
    let collection = db.collection::<PaymasterUserOperations>(&collection_name);

    let mut start_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;

    println!("Monitoring for new documents...");

    loop {
        let filter = doc! { "createdAt": { "$gt": start_time } };
        let start_time_readable = DateTime::<Utc>::from_timestamp(start_time / 1000, (start_time % 1000) as u32 * 1_000_000)
            .expect("Invalid timestamp");
        println!("Current start_time: {} ({})", start_time, start_time_readable);
        println!("Filter: {:?}", filter);
        let mut cursor = collection.find(filter, None).await?;

        while let Some(doc) = cursor.try_next().await? {
            let created_at = DateTime::<Utc>::from_timestamp(doc.created_at / 1000, (doc.created_at % 1000) as u32 * 1_000_000)
                .expect("Invalid timestamp");
            let updated_at = DateTime::<Utc>::from_timestamp(doc.updated_at / 1000, (doc.updated_at % 1000) as u32 * 1_000_000)
                .expect("Invalid timestamp");

            println!("New document found:");
            println!("ID: {}", doc.id);
            println!("Chain ID: {}", doc.chain_id);
            println!("Smart Account Address: {}", doc.smart_account_address);
            println!("State: {}", doc.state);
            println!("Created At: {} ({})", doc.created_at, created_at);
            println!("Updated At: {} ({})", doc.updated_at, updated_at);
            println!("Mode: {}", doc.mode);
            println!("User Op Hash: {}", doc.user_op_hash);
            println!("----------------------------------------");

            start_time = doc.created_at;
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
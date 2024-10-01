// src/stream_app.rs
use mongodb::{bson::doc, Client, options::ChangeStreamOptions};
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;
// use tokio;

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

pub async fn run() -> mongodb::error::Result<()> {
    dotenv::dotenv().ok();
    let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let mongodb_database = std::env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE must be set");
    let mongodb_collection = std::env::var("MONGODB_COLLECTION").expect("MONGODB_COLLECTION must be set");

    let client = Client::with_uri_str(&mongodb_uri).await?;
    let db = client.database(&mongodb_database);
    let collection = db.collection::<PaymasterUserOperations>(&mongodb_collection);

    // Create a change stream to listen for new documents
    let mut change_stream = collection.watch(None, ChangeStreamOptions::default()).await?;
    println!("Streaming new documents...");

    while let Some(change) = change_stream.next().await {
        match change {
            Ok(event) => {
                println!("********************************");
                println!("New change detected: {:?}", event);
                println!("********************************");
            }
            Err(e) => eprintln!("Error watching changes: {}", e),
        }
    }

    Ok(())
}

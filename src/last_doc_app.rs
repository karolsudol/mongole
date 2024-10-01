// src/last_doc_app.rs
use mongodb::{bson::doc, Client, options::FindOptions};
// use tokio;
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;
use chrono::{TimeZone, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct UserOp {
    sender: String,
    nonce: String,
    init_code: String,
    call_data: String,
    signature: String,
    #[serde(rename = "paymasterAndData")]
    paymaster_and_data: String,
    #[serde(rename = "maxFeePerGas")]
    max_fee_per_gas: String,
    #[serde(rename = "maxPriorityFeePerGas")]
    max_priority_fee_per_gas: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PaymasterUserOperations {
    #[serde(rename = "_id")]
    id: String,
    api_key: String,
    request_id: String,
    chain_id: i32,
    #[serde(rename = "smartAccountAddress")]
    smart_account_address: String,
    user_op: UserOp,
    #[serde(rename = "entryPointAddress")]
    entry_point_address: String,
    state: String,
    #[serde(rename = "createdAt")]
    created_at: i64,
    #[serde(rename = "updatedAt")]
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

    let find_options = FindOptions::builder().sort(doc! { "createdAt": -1 }).limit(1).build();
    let mut cursor = collection.find(None, find_options).await?;

    if let Some(result) = cursor.next().await {
        match result {
            Ok(last_record) => {
                println!("********************************");
                // Print the full record in JSON format
                println!("{:?}", last_record);
                println!("********************************");

                // Convert the timestamps from milliseconds (epoch) to human-readable UTC format
                let created_at = Utc.timestamp_millis_opt(last_record.created_at);
                println!("Created At: {}", created_at.unwrap().to_rfc2822());

                let updated_at = Utc.timestamp_millis_opt(last_record.updated_at);
                println!("Updated At: {}", updated_at.unwrap().to_rfc2822()); 

                println!("********************************");
            },
            Err(e) => eprintln!("Error retrieving the last record: {}", e),
        }
    } else {
        println!("No records found.");
    }

    Ok(())
}

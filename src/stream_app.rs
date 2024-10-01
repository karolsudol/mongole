use mongodb::{bson::doc, Client, options::ChangeStreamOptions};
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;

#[derive(Debug, Serialize, Deserialize)]
struct UserOp {
    sender: Option<String>,
    nonce: Option<String>,
    init_code: Option<String>,
    call_data: Option<String>,
    signature: Option<String>,
    paymaster_and_data: Option<String>,
    max_fee_per_gas: Option<String>,
    max_priority_fee_per_gas: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PaymasterUserOperations {
    #[serde(rename = "_id")]
    id: Option<String>,
    actual_gas_cost: Option<i64>,
    actual_gas_used: Option<i64>,
    api_key: Option<String>,
    request_id: Option<String>,
    chain_id: Option<i32>,
    entry_point_address: Option<String>,
    exchange_rate: Option<String>,
    mode: Option<String>,
    paymaster_and_data: Option<String>,
    nonce: Option<i64>,
    oracle_aggregator: Option<String>,
    paymaster_address: Option<String>,
    paymaster_id: Option<String>,
    paymaster_type: Option<String>,
    paymaster_version: Option<String>,
    price_markup: Option<String>,
    price_source: Option<bool>,
    smart_account_address: Option<String>,
    state: Option<String>,
    success: Option<bool>,
    token: Option<String>,
    token_decimal: Option<i32>,
    token_symbol: Option<String>,
    transaction_hash: Option<String>,
    user_op: Option<UserOp>,
    user_op_hash: Option<String>,
    value: Option<String>,
    value_in_usd: Option<String>,
    created_at_timestamp: Option<i64>,
    created_at: Option<i64>,
    updated_at: Option<i64>,
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
                println!("New change detected:");

                if let Some(full_document) = event.full_document {
                    // Filter only known fields in the schema and pretty-print them
                    println!("{:#?}", full_document); // Pretty-print known fields
                } else {
                    // Handle updates without full document, printing only updated fields
                    if let Some(update_desc) = event.update_description {
                        println!("Updated fields: {:#?}", update_desc.updated_fields); // Pretty-print updated fields
                    }
                }

                println!("********************************");
            }
            Err(e) => eprintln!("Error watching changes: {}", e),
        }
    }

    Ok(())
}

use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
use dotenv::dotenv;
use std::env;
use mongodb::bson::Document;
use chrono::{DateTime, Utc};
use futures::StreamExt;

pub async fn run() -> mongodb::error::Result<()> {
    dotenv().ok();
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let mongodb_database = env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE must be set");
    let mongodb_collection = env::var("MONGODB_COLLECTION").expect("MONGODB_COLLECTION must be set");

    let mut client_options = ClientOptions::parse(&mongodb_uri).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    let db = client.database(&mongodb_database);
    
    // Ping the server to see if you can connect to the cluster
    db.run_command(doc! {"ping": 1}, None).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    println!("use('{}');", mongodb_database);

    let collection = db.collection::<Document>(&mongodb_collection);

    let mut cursor = collection.find(None, mongodb::options::FindOptions::builder()
        .sort(doc! { "createdAt": -1 })
        .limit(1)
        .build())
        .await?;

    if let Some(result) = cursor.next().await {
        match result {
            Ok(last_record) => {
                println!("********************************");
                println!("{:#?}", last_record);
                println!("********************************");
                if let (Some(created_at), Some(updated_at)) = (last_record.get("createdAt"), last_record.get("updatedAt")) {
                    if let (Some(created_at), Some(updated_at)) = (created_at.as_i64(), updated_at.as_i64()) {
                        let created_at: DateTime<Utc> = DateTime::from_timestamp_millis(created_at).unwrap();
                        let updated_at: DateTime<Utc> = DateTime::from_timestamp_millis(updated_at).unwrap();

                        println!("Human-readable timestamps:");
                        println!("Created At: {}", created_at.to_rfc2822());
                        println!("Updated At: {}", updated_at.to_rfc2822());
                        println!("********************************");
                    }
                }
            },
            Err(e) => eprintln!("Error retrieving the last record: {}", e),
        }
    } else {
        println!("No records found.");
    }

    Ok(())
}
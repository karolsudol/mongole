mod last_doc_app;
mod stream_app;
mod ping_db;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the app to run: `last_doc`, `stream`, or `ping_db`");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "last_doc" => last_doc_app::run().await?,
        "stream" => {
            // Check for the event type argument
            let event_type = match args.get(2).map(String::as_str) {
                Some("insert") => stream_app::ChangeEventType::Insert,
                Some("update") => stream_app::ChangeEventType::Update,
                Some("all") | None => stream_app::ChangeEventType::All, // Default to 'all' if no second argument is provided
                _ => {
                    eprintln!("Invalid event type. Use `insert`, `update`, or `all`.");
                    std::process::exit(1);
                }
            };
            stream_app::run(event_type).await?; // Pass the event type to the run function
        },
        "ping_db" => ping_db::run().await?,
        _ => {
            eprintln!("Invalid argument. Use `last_doc`, `stream`, or `ping_db`");
            std::process::exit(1);
        }
    }

    Ok(())
}

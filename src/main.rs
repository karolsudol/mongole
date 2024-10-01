mod last_doc_app;
mod stream_app;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the app to run: `last_doc` or `stream`");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "last_doc" => last_doc_app::run().await?,
        "stream" => stream_app::run().await?,
        _ => {
            eprintln!("Invalid argument. Use `last_doc` or `stream`");
            std::process::exit(1);
        }
    }

    Ok(())
}

mod cli;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    match cli::run().await {
        Ok(_) => println!("Success!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

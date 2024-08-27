use dotenv::dotenv;
use std::env;

mod errors;
mod languages;
mod translate;

use translate::translate_text;

#[tokio::main]
async fn main() {
    dotenv().ok();

    match env::var("API_KEY") {
        Ok(api_key) => {
            translate(api_key).await;
        }
        Err(e) => eprintln!("Couldn't read API_KEY: {}", e),
    }
}

async fn translate(api_key: String) {
    let result = translate_text(
        "Hello, world!",
        Some("en"),
        Some("es"),
        false,
        api_key.trim(),
    )
    .await;
    match result {
        Ok(res) => println!("Translation: {}", res.text),
        Err(err) => println!("Error: {}", err),
    }
}

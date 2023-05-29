use clap::Parser;
use reqwest::{Client, Response};
use std::error::Error;
use std::time::{Instant, Duration};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long, default_value_t = 80)]
    port: u8,

    #[arg(short, long, default_value = "GET")]
    method: String,

    #[arg(short, long, default_value_t = 1)]
    requests: u8,

    #[arg(short, long, default_value_t = 1)]
    concurrency: u8,
}

async fn make_request(url: &str, method: &str) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let response = client.request(reqwest::Method::from_bytes(method.as_bytes())?, url).send().await?;
    Ok(response)
}

fn print_request_info(response: &Response, method: &str, duration: Duration) {
    println!("Host: {}", response.url().host().unwrap());
    println!("Method: {}", method);
    println!("Response Duration: {:?}", duration);
    println!();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Retrieve the values from parsed arguments
    let url = args.url;
    let method = args.method;

    // Make the request
    let start_time = Instant::now();
    let response = make_request(&url, &method).await?;
    let duration = start_time.elapsed();

    print_request_info(&response, &method, duration);

    // Handle the response as needed
    // ...

    Ok(())
}

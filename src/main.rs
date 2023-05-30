use clap::Parser;
use reqwest:: Response;
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

    #[arg(short, long, default_value = "")]
    json_data: String,

    #[arg(short, long, default_value_t = 1)]
    concurrency: u8,
}

fn print_request_info(response: &Response, method: &str, duration: Duration) {
    println!("Host: {}", response.url().host().unwrap());
    println!("Method: {}", method);
    println!("Response Duration: {:?}", duration);
}
pub async fn get_method(url : &str)-> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send()
        .await?;
    Ok(response)
}

pub async fn post_method(url : &str,json_data : &str)-> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.post(url)
        .body(json_data.to_owned())
        .header("Content-Type","application/json")
        .send()
        .await?;

    Ok(response)
}

pub async fn delete_method(url : &str,json_data : &str)-> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.delete(url)
        .body(json_data.to_owned())
        .header("Content-Type","application/json")
        .send()
        .await?;
    Ok(response)
}

pub async fn put_method(url : &str,json_data : &str)-> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.put(url)
        .body(json_data.to_owned())
        .header("Content-Type","application/json")
        .send()
        .await?;
    Ok(response)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
     let args = Args::parse();
    // // Retrieve the values from parsed arguments
     let url = args.url;
     let method = args.method;
     let json_data = args.json_data;
     let response;
    // Make the request
    let start_time = Instant::now();
    response = match method.to_uppercase().as_str() {
        "GET" =>  get_method(&url).await?,
        "POST" => post_method(&url,&json_data).await?,
        "DELETE" =>  delete_method(&url,&json_data).await?,
        "PUT" =>  put_method(&url,&json_data).await?,
        _ =>    {eprintln!("Error: {:#?}", "Invalid Method");
                std::process::exit(1)},
    };
    let duration = start_time.elapsed();
    print_request_info(&response, &method, duration);
    Ok(())
}

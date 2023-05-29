use clap::Parser;
use reqwest::{Client, Response};
use std::error::Error;
use std::time::{Instant, Duration};
use serde::{Serialize, Deserialize};

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

//JSON FORMAT
#[derive(Deserialize)]
struct response_Person{
    id : i32,
  username : String,
  email: String,
  firstName: String,
  lastName: String,
  gender: String,
  image: String,
  token: String,
}

pub async fn get_method(url : &str)-> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(url).send()
        .await?;

    Ok(body)
}


pub async fn post_method(url : &str,json_data : &str)-> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    //Return text JSON
    let body = client.post(url)
        .body(json_data.to_owned())
        .header("Content-Type","application/json")
        .send()
        .await?;

        //Return deserialize JSON
        // let body = client.post(url)
        // .body(json_data.to_owned())
        // .header("Content-Type","application/json")
        // .send()
        // .await?
        // .json::<response_Person>()
        // .await?;

        // println!("{:?}",body.id);

    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
     let args = Args::parse();

    // // Retrieve the values from parsed arguments
     let url = args.url;
     let method = args.method;

    println!("{:?}",url);

    // Make the GET request
    let start_time = Instant::now();
    let response = get_method(&url).await?;
    let duration = start_time.elapsed();

     print_request_info(&response, &method, duration);

    //POST POST METHOD
    //url : https://dummyjson.com/auth/login
    // let json_data = r#"{"username": "kminchelle", "password": "0lelplR"}"#;
    // let start_time = Instant::now();
    // let response = post_method(&url,json_data).await?;
    // let duration = start_time.elapsed();

    //  print_request_info(&response, &method, duration);


    Ok(())
}

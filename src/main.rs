use clap::Parser;
use serde::{Deserialize, Serialize};
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



//Struct for JSON FORMAT uncomment to use
// #[derive(Serialize, Deserialize, Debug)]
// struct PersonResponse {
//     id: i32,
//     username: String,
//     email: String,
//     firstName: String,
//     lastName: String,
//     gender: String,
//     image: String,
//     token: String,
// }

pub async fn get_method(url : String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(url).send()
	    .await?
        .text()
        .await?;
    Ok(body)
}

pub async fn post_method(url : &str,json_data : &str) -> Result<String, Box<dyn std::error::Error>> {

    // let json_data = r#"{"username": "kminchelle", "password": "0lelplR"}"#;//JSON DATA

    let client = reqwest::Client::new();


    let res = client.post(url)
    .header("Content-Type","application/json")
    .body(json_data.to_owned())
    .send()
    .await?;

    //JSON deserialization uncomment to use
    //  let i : PersonResponse = res.json::<PersonResponse>().await.unwrap();
    //  println!("{:?}",i.id);//id
    //  println!("{:?}",i.username);//username
    //  println!("{:?}",i.email);//email
    
    Ok(res.text().await?)
}

#[tokio::main]
async fn main() {
    /let args = Args::parse();
    //POST METHOD TEST
    // let data = r#"{"username": "kminchelle", "password": "0lelplR"}"#;
	//  let a =post_method("https://dummyjson.com/auth/login",data).await;
    //  println!("{:?}",a);
}
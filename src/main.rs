use clap::Parser;
use reqwest:: Response;
use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;
use std::time::{Instant, Duration};
use tokio::task;
use futures::future::join_all;
use reqwest::blocking::multipart;

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

    #[arg(short, long, default_value = "")]
    file: String,

    #[arg(short, long, default_value_t = 1)]
    concurrency: u8,
}

fn print_headers(headers: &HeaderMap) {
    for (name, value) in headers {
        if let Ok(value_str) = value.to_str() {
            println!("   {}: {}", name.as_str(), value_str);
        } else {
            if let Some(value_bytes) = value.as_bytes().get(..) {
                println!("   {}: {:?}", name.as_str(), value_bytes);
            } else {
                println!("   {}: <binary data>", name.as_str());
            }
        }
    }
}

fn print_request_info(response_code: u16, response_text: String, response_headers: HeaderMap, duration: Duration) {
    println!("Response Code: {}", response_code);
    println!("Response Headers:");
    print_headers(&response_headers);
    println!("Response Content: {}", response_text);
    println!("Response Duration: {:.3?}", duration);
}

pub async fn get_method(url: &str) -> Result<(String, u16, HeaderMap), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let code = response.status().as_u16();
    let headers = response.headers().clone();
    let text = response.text().await?;
    Ok((text, code, headers))
}

pub async fn post_method(url: &str, json_data: &str,file: &str) -> Result<(String, u16, HeaderMap), Box<dyn std::error::Error>> {
    let code ;
    let headers;
    let text ;

   
    if !file.is_empty()
    {
        let bio = multipart::Part::text(json_data.to_owned())
        .mime_str("application/json")?;

        let form = multipart::Form::new()
        .part("data",bio)
        .file("file", file)?;

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(url)
            .multipart(form)
            .send()?;

        code = response.status().as_u16();
        headers = response.headers().clone();
        text = response.text()?;

    }
    else
    {
        let client = reqwest::Client::new();

        let response = client
        .post(url)
        .body(json_data.to_owned())
        .header("Content-Type", "application/json")
        .send()
        .await?;

     code = response.status().as_u16();
     headers = response.headers().clone();
     text = response.text().await?;

    }


    Ok((text, code, headers))
}

pub async fn delete_method(url: &str, json_data: &str) -> Result<(String, u16, HeaderMap), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .delete(url)
        .body(json_data.to_owned())
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let code = response.status().as_u16();
    let headers = response.headers().clone();
    let text = response.text().await?;
    Ok((text, code, headers))
}

pub async fn put_method(url: &str, json_data: &str,file: &str) -> Result<(String, u16, HeaderMap), Box<dyn std::error::Error>> {
    let code ;
    let headers;
    let text ;

   
    if !file.is_empty()
    {
        let bio = multipart::Part::text(json_data.to_owned())
        .mime_str("application/json")?;

        let form = multipart::Form::new()
        .part("data",bio)
        .file("file", file)?;

        let client = reqwest::blocking::Client::new();
        let response = client
            .put(url)
            .multipart(form)
            .send()?;

        code = response.status().as_u16();
        headers = response.headers().clone();
        text = response.text()?;

    }
    else
    {
        let client = reqwest::Client::new();

        let response = client
        .put(url)
        .body(json_data.to_owned())
        .header("Content-Type", "application/json")
        .send()
        .await?;

     code = response.status().as_u16();
     headers = response.headers().clone();
     text = response.text().await?;

    }


    Ok((text, code, headers))
}
    
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
     let args = Args::parse();
    // // Retrieve the values from parsed arguments
     let url = args.url;
     let method = args.method.to_uppercase();
     let json_data = args.json_data;
     let file = args.file;
     //JSON Validation
     if method != "GET"
     {
        let _json : serde_json::Value =serde_json::from_str(&json_data[..]).expect("JSON was not well-formatted");
     }
    let start_time = Instant::now();
    let concurrency = args.concurrency as usize;
    let mut tasks = Vec::new();

    for _ in 0..concurrency {
        let url = url.clone();
        let json_data = json_data.clone();
        let method = method.clone();
        let file = file.clone();

        let task = task::spawn(async move {
            let result = match method.as_str() {
                "GET" => get_method(&url).await,
                "POST" => post_method(&url, &json_data, &file).await,
                "DELETE" => delete_method(&url, &json_data).await,
                "PUT" => put_method(&url, &json_data, &file).await,
                _ => {
                    eprintln!("Error: {:#?}", "Invalid Method");
                    std::process::exit(1);
                }
            };
            match result {
                Ok((response_text, response_code, response_headers)) => {
                    Some((response_text, response_code, response_headers))
                }
                Err(err) => {
                    eprintln!("Error: {:#?}", err);
                    None
                }
            }        });
        tasks.push(task);
    }

    let results = join_all(tasks).await;
    let duration = start_time.elapsed();

    for result in results {
        if let Ok(Some((response_text, response_code, response_headers))) = result {
            print_request_info(response_code, response_text, response_headers, duration);
        }
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_request() {
        let url = "http://scooterlabs.com/echo";
        let (_response_text, response_code, _) = get_method(url).await.expect("Failed to make the request");
        assert_eq!(response_code, 200);
        
    }

    #[tokio::test]
    async fn get_request_with_invalid_url() {
        let url = "http://scooterlabs.cm/echo";
        assert!(get_method(&url).await.is_err());
    }

    #[tokio::test]
    async fn post_request() {
        let url = "http://scooterlabs.com/echo";
        let json_data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        let (_response_text, response_code, _) = post_method(url, json_data,"").await.expect("Failed to make the request");
        assert_eq!(response_code, 200);
        
    }

    #[tokio::test]
    async fn delete_request() {
        let url = "http://scooterlabs.com/echo";
        let json_data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        let (_response_text, response_code, _) = delete_method(url, json_data).await.expect("Failed to make the request");
        assert_eq!(response_code, 200);
        
    }

    #[tokio::test]
    async fn put_request() {
        let url = "http://scooterlabs.com/echo";
        let json_data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        let (_response_text, response_code, _) = put_method(url, json_data,"").await.expect("Failed to make the request");
        assert_eq!(response_code, 200);
    }

    #[tokio::test]
    async fn post_request_with_file() {
        //New thread
        let handle = tokio::spawn(async {
            let url = "http://scooterlabs.com/echo";
            let filename = "windowsImage.jpg";
            let json_data = r#"
            {
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ]
            }"#;
            let (_response_text, response_code, _) = post_method(url, json_data, filename).await.expect("Failed to make the request");
            assert_eq!(response_code, 200);
        });
        let _ = handle.await;
    }

    #[tokio::test]
    async fn put_request_with_file() {
        //New thread
        let handle = tokio::spawn(async {
            let url = "http://scooterlabs.com/echo";
            let filename = "windowsImage.jpg";
            let json_data = r#"
            {
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ]
            }"#;
            let (_response_text, response_code, _) = put_method(url, json_data, filename).await.expect("Failed to make the request");
            assert_eq!(response_code, 200);
        });
        let _ = handle.await;
    }
}

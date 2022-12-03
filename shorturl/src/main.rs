use lambda_runtime::{LambdaEvent, Error};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Output {
    message: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct ShortURLs {
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
   let handler = lambda_runtime::service_fn(handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<serde_json::Value>) -> Result<Output, Error> {
    let ( payload, _context )  = event.into_parts(); 
    let body = payload["body"].as_str().unwrap() ;
    let input = serde_json::from_str::<ShortURLs>(body).unwrap() ;
    print!("Welcome, {}!", payload);
    let message = format!("URL, {}!", input.url);
    Ok(Output {
        message: message,
    })
}

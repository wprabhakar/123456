use lambda_runtime::{LambdaEvent, Error};
use serde::{Serialize, Deserialize};
use nanoid::nanoid;

#[derive(Serialize)]
struct Output {
    url: String,
    shortenUrl: String
}
#[derive(Debug, Serialize, Deserialize)]
struct ShortURLs {
    url: String,
}

pub const ALPHA_NUMERIC: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  ];

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::service_fn(newlink);
    lambda_runtime::run(handler).await?;
    Ok(())
}

async fn newlink(event: LambdaEvent<serde_json::Value>) -> Result<Output, Error> {
    let ( payload, _context )  = event.into_parts(); 
    print!("Payload {}!", payload);
    let body = payload["body"].as_str().unwrap() ;
    let input = serde_json::from_str::<ShortURLs>(body).unwrap() ;
    let url = format!("url {}", input.url);
    let short_url = format!("{}", nanoid!(9, &ALPHA_NUMERIC));
    Ok(Output {
        url: url,
        shortenUrl: short_url,
    })
}

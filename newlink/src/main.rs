//use lambda_runtime::{LambdaEvent, Error};
use lambda_http::{service_fn, Error, Context, Body, IntoResponse, Request, Response, RequestExt};
use serde::{Serialize, Deserialize};
use nanoid::nanoid;
use serde_json::{json};

// #[derive(Serialize)]
// struct Output {
//     url: String,
//     shortenUrl: String
// }

#[derive(Debug, Deserialize, Default, PartialEq)]
struct ShortURLs {
    #[serde(default)]
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
    let handler = lambda_http::service_fn(newlink);
    lambda_http::run(handler).await?;
    Ok(())
}

async fn newlink(request: Request) -> Result<serde_json::Value, Error>{
    let input: ShortURLs = request.payload().unwrap_or_else(|_parse_err| None).unwrap_or_default();
    print!("Input {:?}", input);
    if input.url.is_empty() == true {
        Ok(json!({
            "statusCode": 400,
            "body": { "error": "missing url" },
        }))
    }
    else
    {
        let short_url = format!("{}", nanoid!(9, &ALPHA_NUMERIC));
        Ok(json!({
            "statusCode": 200,
            "body": { "url": input.url, "shortenUrl": short_url },
        }))
   }
    //     Ok(Output {
    //     url: url,
    //     shortenUrl: short_url,
    // })
}

// async fn newlink(event: LambdaEvent<serde_json::Value>) -> Result<Output, Error> {
//     let ( payload, _context )  = event.into_parts(); 
//     print!("Payload {}!", payload);
//     let body = payload["body"].as_str().unwrap() ;
//     let input = serde_json::from_str::<ShortURLs>(body).unwrap() ;
//     let url = format!("url {}", input.url);
//     let short_url = format!("{}", nanoid!(9, &ALPHA_NUMERIC));
//     Ok(Output {
//         url: url,
//         shortenUrl: short_url,
//     })
// }

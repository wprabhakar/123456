use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use lambda_runtime::{LambdaEvent, Error};
use serde::{Serialize, Deserialize};

// async fn newlink(event: LambdaEvent<serde_json::Value>) -> Result<Output, Error> {
//     let ( payload, _context )  = event.into_parts(); 
//     print!("Payload  {}!", payload);
//     let body = payload["body"].as_str().unwrap() ;
//     let input = serde_json::from_str::<ShortURLs>(body).unwrap() ;
//     let url = format!("url {}", input.url);
//     let short_url = format!("{}", nanoid!(9, &ALPHA_NUMERIC));
//     Ok(Output {
//         url: url,
//         shortenUrl: short_url,
//     })
// }

async fn getlink(event: LambdaEvent<serde_json::Value>) -> Result<Response<Body>, Error> {
    let ( payload, _context )  = event.into_parts(); 
    print!("Payload  {}!", payload);
    let resp = Response::builder()
        .status(301)
        .header("location", "http://www.google.com".into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::service_fn(newlink);
    lambda_runtime::run(handler).await?;
    Ok(())
}
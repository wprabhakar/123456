use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_runtime::{LambdaEvent, Error};
use serde::{Serialize, Deserialize};

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
    let handler = lambda_runtime::service_fn(getlink);
    lambda_runtime::run(handler).await?;
    Ok(())
}

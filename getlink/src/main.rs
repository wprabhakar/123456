use serde_json::{json};
use lambda_runtime::{
    Error as LambdaError,
    LambdaEvent,
};
 
type LambdaResult<T> = Result<T, LambdaError>;
 
async fn getlink(event: LambdaEvent<serde_json::Value>) -> Result<serde_json::Value, lambda_runtime::Error> {
    let ( payload, _context )  = event.into_parts(); 
    print!("**Payload  {}!", payload);
    Ok(json!({
        "statusCode": 302,
        "headers": { "location": "http://www.google.com", "content-type": "text/html" },
    }))
}

#[tokio::main]
async fn main() -> LambdaResult<()>  {
    let handler = lambda_runtime::service_fn(getlink);
    lambda_runtime::run(handler).await?;
    Ok(())
}

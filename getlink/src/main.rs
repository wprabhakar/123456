//use lambda_http::{ Body, Error };
//use http::{Request, Response, StatusCode};
//use lambda_runtime::{service_fn,LambdaEvent};
// use serde::{Serialize, Deserialize};
use serde_json::{json};
// use lambda_apigateway_response::{
//     http::StatusCode,
//     types::{
//         Headers,
//         MultiValueHeaders,
//     },
//     Response,
// };
use lambda_runtime::{
    Error as LambdaError,
    LambdaEvent,
};
 
type LambdaResult<T> = Result<T, LambdaError>;
 
async fn getlink(event: LambdaEvent<serde_json::Value>) -> Result<serde_json::Value, lambda_runtime::Error> {
    let ( payload, _context )  = event.into_parts(); 
    print!("Payload  {}!", payload);
    Ok(json!({
        "statusCode": 200,
        "headers": { "location": "http://www.google.com"},
        "body": "OK",
    }))
    // let resp = Response {
    //     status_code: StatusCode::,
    //     body: json!({
    //         "message": "Hello world!",
    //     }),
    //     headers: Headers::new(),
    //     multi_value_headers: MultiValueHeaders::new(),
    //     is_base64_encoded: true,
    // };


    // Ok(resp)
}

#[tokio::main]
async fn main() -> LambdaResult<()>  {
    let handler = lambda_runtime::service_fn(getlink);
    lambda_runtime::run(handler).await?;
    Ok(())
}

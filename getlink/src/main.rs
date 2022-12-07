use lambda_http::{service_fn, Request, Error};
use serde::{Serialize, Deserialize};
use aws_lambda_events::encodings::Body;

use aws_sdk_dynamodb::{
    model::AttributeValue,
    Client, Error as dynamodbError
};
use http::{Uri, Response};
use aws_smithy_http::endpoint::Endpoint;


#[derive(Debug, Serialize, Deserialize)]
pub struct ShortURLItem {
    pub url: String,
    pub slink: String,
}

pub async fn get_url ( client: &Client, table_name: &str, slink: &str) -> Result<ShortURLItem, dynamodbError> {
    let req = client
        .query()
        .table_name(table_name)
        .index_name("slink_gsi")
        .key_condition_expression("slink = :hashKey",)
        .expression_attribute_values(":hashKey",AttributeValue::S(slink.to_string()),);
//    dbg!(&req);
    let mut o:ShortURLItem = ShortURLItem { url: String::from(""), slink: slink.to_string()};
    match req.send().await
    {
      Ok(resp) => {
        let m = resp.items().unwrap().to_vec();
        if m.len() > 0 {
            o.url = m[0].get("url").unwrap().as_s().unwrap().to_string() ;
        }
      },
      Err(err) => {
        println!("Failed {:?}", err);
      },
    };
    println!(" {:?}", o) ;
    Ok(o) 
}

async fn getlink(client: &aws_sdk_dynamodb::Client, table_name: String, event: Request) -> Result<Response<Body>, Error> {
    let ( payload, _context )  = event.into_parts(); 
    let slink = payload.uri.path().strip_prefix("/").unwrap().trim_end_matches("/");
    print!("path {:?}", slink) ;

    let res = get_url(&client, &table_name, slink).await?;
    if res.url == "".to_string() {
        Ok(Response::builder()
        .status(404)
        .body(slink.into())
        .expect("failed to render response"))
    }
    else {
        Ok(Response::builder()
        .status(302)
        .header("content-type", "text/html")
        .header("location", res.url.as_str())
        .body("location: google.com".into())
        .expect("failed to render response"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Error>  {
    // let endpoint = Endpoint::immutable(Uri::from_static("http://localhost:8000"));
    // let conf = aws_config::from_env().endpoint_resolver(endpoint).load().await;
    let conf = aws_config::from_env().load().await;
    let client = aws_sdk_dynamodb::Client::new(&conf);
 
    let table_name = "shorturls-table".to_string();
    lambda_http::run(service_fn(|event: Request| getlink(&client, table_name.clone(), event))).await?;
    Ok(())
}

use aws_config::meta::region::RegionProviderChain;
use http::Uri;
use aws_smithy_http::endpoint::Endpoint;

use lambda_http::{service_fn, Request, RequestExt, Error};
use serde::{Serialize, Deserialize};
use nanoid::nanoid;
use serde_json::{json};

#[derive(Debug, Deserialize, Default, PartialEq)]
struct ShortURL {
    #[serde(default)]
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortURLItem {
    pub url: String,
    pub slink: String,
}

pub const ALPHA_NUMERIC: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  ];

use aws_sdk_dynamodb::{
    model::{
        AttributeDefinition, BillingMode, KeySchemaElement, AttributeValue,
        KeyType, ScalarAttributeType, Projection, ProjectionType, GlobalSecondaryIndex
    },
    Client, Error as dynamodbError
};
/*
async fn create_table_if_not_exists(client: &Client, table_name: &str) -> Result<(), dynamodbError> 
{
    let res = client.list_tables().send().await?;
    if res.table_names().unwrap().contains(&table_name.to_string()) == false {
        println!("Table {:?} does not exist.  creating.", table_name) ;
        create_table( &client, &table_name).await? ;
    }
    else {
        println!("Table {:?} exist.", table_name) ;
    }
    Ok(())
}


async fn create_table(client: &Client, table_name: &str) -> Result<(), dynamodbError> {
    let pkurl = AttributeDefinition::builder()
        .attribute_name("url")
        .attribute_type(ScalarAttributeType::S)
        .build();

    let pkslink = AttributeDefinition::builder()
        .attribute_name("slink")
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ksurl = KeySchemaElement::builder()
        .attribute_name("url")
        .key_type(KeyType::Hash)
        .build();

    let ksslink = KeySchemaElement::builder()
        .attribute_name("slink")
        .key_type(KeyType::Hash)
        .build();

    let proj = Projection::builder()
    .set_projection_type(Some(ProjectionType::All)).build();

    let gsislink: GlobalSecondaryIndex = GlobalSecondaryIndex::builder()
        .index_name("slink_gsi")
        .set_key_schema(Some(vec!(ksslink))) 
        .set_projection(Some(proj))
        .build();

    client
        .create_table()
        .table_name(String::from(table_name))
        .key_schema(ksurl)
        .global_secondary_indexes(gsislink)
        // .key_schema(ksslink)
        .attribute_definitions(pkurl)
        .attribute_definitions(pkslink)
        .billing_mode(BillingMode::PayPerRequest)
        .send()
        .await?;
    Ok(())
}
 */

pub async fn add_item(client: &Client, item: &ShortURLItem, table_name: &str) -> Result<(), dynamodbError> {
    let request = client
        .put_item()
        .table_name(table_name)
        .item("slink", AttributeValue::S(
            item.slink.to_string()))
        .item("url", AttributeValue::S(
            item.url.to_string()))
        ;
    println!("storing {:?}", item);
    request.send().await?;
    // print!("PutItem {:?}", resp) ;
    Ok(())
}


pub async fn get_slink ( client: &Client, table_name: &str, url: &str) -> Result<ShortURLItem, dynamodbError> {
    let item = client.get_item()
        .table_name(table_name)
        .key("url",AttributeValue::S(url.to_string()),)
        .send().await?;
    let mut o:ShortURLItem = ShortURLItem { url: url.to_string(), slink: String::from("")};
    if item.item().is_none() {
        Ok(o) 
    }
    else
    {        
        let attributes = item.item().unwrap();
        let f = format!("{}", attributes.get("slink").unwrap().as_s().unwrap());
        print!("{f:?}");
        o.slink = f;
        Ok(o)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    // let endpoint = Endpoint::immutable(Uri::from_static("http://localhost:8000"));
    // let conf = aws_config::from_env().endpoint_resolver(endpoint).load().await;
    let conf = aws_config::from_env().load().await;
    let client = aws_sdk_dynamodb::Client::new(&conf);
 
    let table_name = "shorturls-table".to_string();

    lambda_http::run(service_fn(|event: Request| newlink(&client, table_name.clone(), event))).await?;
    Ok(())
}


pub async fn newlink(client: &aws_sdk_dynamodb::Client, table_name: String, request: Request) -> Result<serde_json::Value, dynamodbError>{
    let input: ShortURL = request.payload().unwrap_or_else(|_parse_err| None).unwrap_or_default();
    print!("*Payload {:?}", input);
    if input.url.is_empty() == true {
        Ok(json!({"error": "missing url" }))
    }
    else
    {
//        create_table_if_not_exists(&client, &table_name).await?;
        let res = get_slink(&client, &table_name, &input.url).await?;
        print!("{res:?}");
        if res.slink == "".to_string() {
            let short_url = format!("{}", nanoid!(9, &ALPHA_NUMERIC));
            let item = ShortURLItem {
                slink: short_url.to_string(),
                url: input.url,
            };
            print!("storing {:?}", item) ;
            add_item(&client, &item, &table_name).await?;
            Ok(json!({ "url": item.url, "shortenUrl": format!("https://shortenurl.org/{}",short_url) })) 
        }
        else {
            print!("SLINK: {:?}", res.slink.as_str());
            Ok(json!({ "url": input.url, "shortenUrl": res.slink.as_str()})) 
        }
   }
}

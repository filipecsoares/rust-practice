use actix_web::http::Uri;
use aws_sdk_dynamodb::{
    model::{
        AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    },
    Client, Endpoint,
};

pub static TODO_CARD_TABLE: &str = "TODO_CARDS";

pub async fn create_table() {
    let config = aws_config::load_from_env().await;
    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
        .endpoint_resolver(Endpoint::immutable(Uri::from_static(
            "http://localhost:8000",
        )))
        .build();
    let client = Client::from_conf(dynamodb_local_config);
    let table_name = TODO_CARD_TABLE.to_string();
    let ad = AttributeDefinition::builder()
        .attribute_name("id")
        .attribute_type(ScalarAttributeType::S)
        .build();
    let ks = KeySchemaElement::builder()
        .attribute_name("id")
        .key_type(KeyType::Hash)
        .build();
    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(1)
        .write_capacity_units(1)
        .build();

    match client
        .create_table()
        .table_name(table_name)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await
    {
        Ok(output) => {
            println!("Output: {:?}", output);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

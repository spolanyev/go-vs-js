//@author Stanislav Polaniev <spolanyev@gmail.com>

use aws_config;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client as DbClient;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples

#[derive(Debug, Deserialize)]
struct Event {
    #[serde(rename = "queryStringParameters")]
    query_string_parameters: QueryStringParameters,
}

#[derive(Debug, Deserialize)]
struct QueryStringParameters {
    #[serde(rename = "fileId")]
    file_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FileInfo {
    #[serde(rename = "fileId")]
    file_id: String,
    name: String,
    #[serde(rename = "s3Key")]
    s3_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u64>,
}

impl std::fmt::Display for FileInfo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::json!(self).to_string();
        write!(formatter, "{}", json)
    }
}

async fn get_file_info(
    db_client: &DbClient,
    request: LambdaEvent<Event>,
) -> Result<FileInfo, Error> {
    println!("Request: {:?}", request);

    let file_id = request.payload.query_string_parameters.file_id;

    let get_item_output = db_client
        .get_item()
        .table_name("fi-file-sharing")
        .key("fileId", AttributeValue::S(file_id.to_owned()))
        .send()
        .await?;

    let Some(item) = get_item_output.item else {
        println!("Error: item not found");
        return Err(Error::try_from("item not found").unwrap());
    };

    let result = FileInfo {
        file_id: item.get("fileId").unwrap().as_s().unwrap().to_owned(),
        name: item.get("name").unwrap().as_s().unwrap().to_owned(),
        s3_key: item.get("s3Key").unwrap().as_s().unwrap().to_owned(),
        preview: item
            .get("preview")
            .and_then(|v| Option::from(v.as_s().unwrap().to_owned())),
        size: item
            .get("size")
            .and_then(|v| Option::from(v.as_n().unwrap().to_owned().parse::<u64>().unwrap())),
    };

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let config = aws_config::load_from_env().await;
    let db_client = DbClient::new(&config);

    lambda_runtime::run(service_fn(|event: LambdaEvent<Event>| async {
        get_file_info(&db_client, event).await
    }))
    .await
}

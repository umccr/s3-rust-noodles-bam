use lambda_http::{http::StatusCode, IntoResponse, Request, RequestExt, Response, Error, service_fn};
use serde_derive::Deserialize;
use serde_json::json;
use tracing::{event, Level};
use url::Url;

use s3_rust_noodles_bam::{read_bam_header, stream_s3_object_with_params};
use s3_rust_noodles_bam::telemetry::{get_subscriber, init_subscriber};

#[derive(Debug, Deserialize, Default)]
struct EventPayload {
    #[serde(default)]
    bam: String,
    region: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = get_subscriber("get_bam_header".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    lambda_http::run(service_fn(get_bam_header)).await?;
    Ok(())
}

async fn get_bam_header(request: Request) -> Result<impl IntoResponse, Error> {
    let payload: EventPayload = request.payload()
        .unwrap_or(None)
        .unwrap_or_default();

    let s3_object_result = if payload.bam.is_empty() || !payload.bam.starts_with("s3://") {
        event!(Level::INFO, "get_bam_header with default BAM");
        println!("get_bam_header with default BAM");

        Err(stream_s3_object_with_params("foo".to_string(), "bar".to_string(), Some("ap-southeast-2".to_string())).await?)
    } else {
        event!(Level::INFO, "get_bam_header with payload BAM");
        println!("get_bam_header with payload BAM");

        let payload_url = Url::parse(payload.bam.as_str())?;

        let proto = payload_url.scheme();
        let bucket = payload_url.host_str().unwrap();
        let url_path = payload_url.path();
        let key = url_path.strip_prefix('/').unwrap();

        println!("{}", proto);
        println!("{}", bucket);
        println!("{}", key);

        Ok(stream_s3_object_with_params(
            bucket.to_string(),
            key.to_string(),
            payload.region,
        ).await?)
    };

    match s3_object_result {
        Ok(s3_object) => {
            let bam_header = read_bam_header(s3_object).await?;
            Ok(response(
                StatusCode::OK,
                json!({ "message": format!("{}", bam_header)}).to_string(),
            ))
        }

        Err(err) => {
            println!("Error fetching s3 object: {:#?}", err);
            Ok(response(
                StatusCode::BAD_REQUEST,
                json!({ "message": format!("{}", "error")}).to_string(),
            ))
        }
    }
}

/// HTTP Response with a JSON payload
fn response(status_code: StatusCode, body: String) -> Response<String> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}

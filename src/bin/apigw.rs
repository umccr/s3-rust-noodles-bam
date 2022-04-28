use lambda_http::{http::StatusCode, Body, Request, RequestExt, Response, Error, service_fn};
use serde_derive::Deserialize;
use serde_json::json;
use tracing::{event, Level};
use url::Url;

use s3_rust_noodles_bam::{stream_s3_object_with_params};
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

async fn get_bam_header(request: Request) -> Result<Response<Body>, Error> {
    let payload: EventPayload = request.payload()
        .unwrap_or(None)
        .unwrap_or_default();

    if payload.bam.is_empty() || !payload.bam.starts_with("s3://") {
        event!(Level::INFO, "get_bam_header with default BAM");
        
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(lambda_http::Body::Text(json!({"message": "error"}).to_string())).map_err(|err| Error::from(err.to_string()))
    } else {
        event!(Level::INFO, "get_bam_header with payload BAM");

        let payload_url = Url::parse(payload.bam.as_str())?;

        let bucket = payload_url.host_str().unwrap();
        let url_path = payload_url.path();
        let key = url_path.strip_prefix('/').unwrap();

        let bam_header = stream_s3_object_with_params(
            bucket.to_string(),
            key.to_string(),
            payload.region,
        ).await?;

        Response::builder()
            .status(StatusCode::OK)
            .body(lambda_http::Body::Text(json!({ "message": bam_header}).to_string())).map_err(|err| Error::from(err.to_string()))
    }
}
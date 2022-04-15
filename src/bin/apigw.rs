use lambda_http::{http::StatusCode, IntoResponse, Request, RequestExt, Response, service_fn};
use serde_derive::Deserialize;
use serde_json::json;
use tracing::{event, Level};
use url::Url;

use s3_rust_noodles_bam::{read_bam_header, stream_s3_object, stream_s3_object_with_params};
use s3_rust_noodles_bam::telemetry::{get_subscriber, init_subscriber};

type E = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Event payload model
#[derive(Debug, Deserialize, Default)]
struct Bambada {
    #[serde(default)]
    bam: String,
    region: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), E> {
    let subscriber = get_subscriber("get_bam_header".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    lambda_http::run(service_fn(|event: Request| get_bam_header(event))).await?;
    Ok(())
}

async fn get_bam_header(request: Request) -> Result<impl IntoResponse, E> {
    let bambada: Bambada = request.payload()
        .unwrap_or_else(|_parse_err| None)
        .unwrap_or_default();
    // println!("{:#?}", bambada.bam);

    let s3_object_result;

    if bambada.bam.is_empty() || !bambada.bam.starts_with("s3://") {
        event!(Level::INFO, "get_bam_header with default BAM");
        println!("get_bam_header with default BAM");

        s3_object_result = stream_s3_object().await;
    } else {
        event!(Level::INFO, "get_bam_header with payload BAM");
        println!("get_bam_header with payload BAM");

        let bambada_url = Url::parse(bambada.bam.as_str())?;
        // println!("{:#?}", bambada_url);

        let proto = bambada_url.scheme();
        let bucket = bambada_url.host_str().unwrap();
        let url_path = bambada_url.path();
        let key = url_path.strip_prefix("/").unwrap();

        println!("{}", proto);
        println!("{}", bucket);
        println!("{}", key);

        s3_object_result = stream_s3_object_with_params(
            bucket.to_string(),
            key.to_string(),
            bambada.region,
        ).await;
    }

    Ok(match s3_object_result {
        Ok(s3_object) => {
            let bam_header = read_bam_header(s3_object).await;
            response(
                StatusCode::OK,
                json!({ "message": format!("{}", bam_header?)}).to_string(),
            )
        }

        Err(err) => {
            println!("Error fetching s3 object: {:#?}", err);
            response(
                StatusCode::BAD_REQUEST,
                json!({ "message": format!("{}", err)}).to_string(),
            )
        }
    })
}

/// HTTP Response with a JSON payload
fn response(status_code: StatusCode, body: String) -> Response<String> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}

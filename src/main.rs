extern crate jemallocator;

use lambda_runtime::{Error, LambdaEvent, service_fn};
use noodles::sam;
use serde_json::{json, Value};
use tracing::{event, Level};

use s3_rust_noodles_bam::{read_bam_header, stream_s3_object};
use s3_rust_noodles_bam::telemetry::{get_subscriber, init_subscriber};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = get_subscriber("lambda_bam_header".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    event!(Level::INFO, "Telemetry registered");
    let func = service_fn(bam_header_as_json);
    lambda_runtime::run(func).await?;
    Ok(())
}

/// Turns header string into a JSON
async fn bam_header_as_json(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let header = s3_read_bam_header(event).await?;

    Ok(json!({ "message": format!("{}", header) }))
}

/// Reads BAM header from returned S3 bytes
async fn s3_read_bam_header(_event: LambdaEvent<Value>) -> Result<sam::Header, Error> {
    // TODO parse _event and route to backend - stream_s3_object() or stream_s3_object_with_params()
    //  accordingly based on event payload
    let s3_object = stream_s3_object().await?;
    let header = read_bam_header(s3_object).await?.parse()?;
    Ok(header)
}

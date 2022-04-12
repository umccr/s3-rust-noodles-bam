extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use bytes::Bytes;
use std::io::{Cursor};
use serde_json::{json, Value};

use lambda_runtime::{service_fn, LambdaEvent, Error};

use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_sdk_s3 as s3;
use s3::Region;

use noodles::bam;
use noodles::sam;
use crate::sam::header::ParseError;

use s3_rust_noodles_bam::telemetry::{get_subscriber, init_subscriber};

// Change these to your bucket, key and region
const BUCKET: &str = "umccr-research-dev";
const KEY: &str = "htsget/htsnexus_test_NA12878.bam";
const REGION: &str = "ap-southeast-2";


#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = get_subscriber("lambda_bam_header".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let func = service_fn(bam_header_as_json);
    lambda_runtime::run(func).await?;
    Ok(())
}

/// Turns header string into a JSON
async fn bam_header_as_json(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let header = s3_read_bam_header(event).await?;

    Ok(json!({ "message": format!("{}", header) }))
}

/// Fetches S3 object
async fn stream_s3_object() -> Result<Bytes, Error> {
    let creds_provider = DefaultCredentialsChain::builder()
            .region(Region::new(REGION))
            .build().await;

    let conf = s3::Config::builder()
        .region(Region::new(REGION))
        .credentials_provider(creds_provider)
        .build();
    let client = s3::Client::from_conf(conf);

    let resp = client.get_object().bucket(BUCKET).key(KEY).send().await?;
    let data = resp.body.collect().await?;

    return Ok(data.into_bytes());
}

/// Reads BAM S3 object header
async fn read_bam_header(bam_bytes: Bytes) -> Result<sam::Header, ParseError> {
    let mut s3_obj_buffer = Cursor::new(bam_bytes.to_vec());
    // Rewind buffer Cursor after writing, so that next reader can consume header data...
    s3_obj_buffer.set_position(0);

    // ... and read the header
    let mut reader = bam::Reader::new(s3_obj_buffer);
    reader.read_header().unwrap().parse()//?.parse::<sam::Header>()
}

/// Reads BAM header from returned S3 bytes
async fn s3_read_bam_header(_event: LambdaEvent<Value>) -> Result<sam::Header, Error> {
    let s3_object = stream_s3_object().await?;
    Ok(read_bam_header(s3_object).await?)
}
use std::io::ErrorKind;
use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_sdk_s3::{Client, Config, Region, types::ByteStream};
use lambda_runtime::Error;
use noodles_bam as bam;
use tokio_util::io::StreamReader;
use tracing::{event, Level};
use futures_util::stream::TryStreamExt;

pub mod telemetry;

const BUCKET: &str = "umccr-research-dev";
const KEY: &str = "htsget/htsnexus_test_NA12878.bam";
const REGION: &str = "ap-southeast-2";

/// Fetches S3 object using default
pub async fn stream_s3_object() -> Result<ByteStream, Error> {
    return stream_s3_object_with_params(BUCKET.to_string(), KEY.to_string(), None).await;
}

/// Fetches S3 object with given params
pub async fn stream_s3_object_with_params(bucket: String, key: String, region: Option<String>) -> Result<ByteStream, Error> {
    let region_ = Region::new(region.unwrap_or_else(|| REGION.to_string()));

    let creds_provider = DefaultCredentialsChain::builder()
        .region(region_.clone())
        .build().await;

    let conf = Config::builder()
        .region(region_.clone())
        .credentials_provider(creds_provider)
        .build();

    let client = Client::from_conf(conf);

    event!(Level::INFO, "Getting S3 object bytes...");

    // Preflight check -- like file size, DEEP_ARCHIVE restore stuff, etc... :)
    // let head_req = client
    //     .head_object()
    //     .bucket(bucket.clone())
    //     .key(key.clone())
    //     .send()
    //     .await?;

    let resp = client
        .get_object()
        .bucket(bucket.clone())
        .key(key.clone())
        .send().await?;

    let data = resp.body;

    Ok(data)
}

/// Reads BAM S3 object header
pub async fn read_bam_header(bam_bytestream: ByteStream) -> std::io::Result<String> {
    let map_err = bam_bytestream.map_err(|err| std::io::Error::new(ErrorKind::Other, err));
    let stream_reader = StreamReader::new(map_err);
    let mut reader = bam::AsyncReader::from(stream_reader);
    reader.read_header().await
}
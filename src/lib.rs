use std::io::Cursor;

use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_sdk_s3::{Client, Config, Region};
use bytes::Bytes;
use lambda_runtime::Error;
use noodles::{bam, sam};
use tracing::{event, Level};

use crate::sam::header::ParseError;

pub mod telemetry;

const FILE_SIZE_LIMIT_IN_BYTES: i64 = 8596799;  // ~8.6 MB

const BUCKET: &str = "umccr-research-dev";
const KEY: &str = "htsget/htsnexus_test_NA12878.bam";
const REGION: &str = "ap-southeast-2";

/// Fetches S3 object using default
pub async fn stream_s3_object() -> Result<Bytes, Error> {
    return stream_s3_object_with_params(BUCKET.to_string(), KEY.to_string(), None).await;
}

/// Fetches S3 object with given params
pub async fn stream_s3_object_with_params(bucket: String, key: String, region: Option<String>) -> Result<Bytes, Error> {
    let region_ = Region::new(region.unwrap_or(REGION.to_string()));

    let creds_provider = DefaultCredentialsChain::builder()
        .region(region_.clone())
        .build().await;

    let conf = Config::builder()
        .region(region_.clone())
        .credentials_provider(creds_provider)
        .build();

    let client = Client::from_conf(conf);

    event!(Level::INFO, "Getting S3 object bytes...");
    println!("Getting S3 object bytes...");

    // Preflight check -- like file size, DEEP_ARCHIVE restore staff, etc... :)
    let head_req = client
        .head_object()
        .bucket(bucket.clone())
        .key(key.clone())
        .send()
        .await?;

    if head_req.content_length > FILE_SIZE_LIMIT_IN_BYTES {
        let msg = format!("Oh shoot, file too large {} bytes to download! We should ideally \
        be slicing it with Noodle! Sorry, try something < 5MB, pls!", head_req.content_length);
        println!("{}", msg);
        return Err(Error::from(msg));
    }

    let resp = client
        .get_object()
        .bucket(bucket.clone())
        .key(key.clone())
        .send().await?;

    let data = resp.body.collect().await?;

    return Ok(data.into_bytes());
}

/// Reads BAM S3 object header
pub async fn read_bam_header(bam_bytes: Bytes) -> Result<sam::Header, ParseError> {
    let mut s3_obj_buffer = Cursor::new(bam_bytes.to_vec());
    // Rewind buffer Cursor after writing, so that next reader can consume header data...
    s3_obj_buffer.set_position(0);

    // ... and read the header
    let mut reader = bam::Reader::new(s3_obj_buffer);
    reader.read_header().unwrap().parse()//?.parse::<sam::Header>()
}

extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use bytes::Bytes;
use std::io::{Cursor};

use lambda_runtime::{ handler_fn, Context, Error };
use serde_json::{ json, Value };

use aws_sdk_s3 as s3;
use s3::Region;

use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::SubscriberBuilder;

use noodles_bam as bam;
use noodles_sam as sam;

// Change these to your bucket, key and region
const BUCKET: &str = "umccr-research-dev";
const KEY: &str = "htsget/htsnexus_test_NA12878.bam";
const REGION: &str = "ap-southeast-2";


#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler_fn(s3_read_bam_header)).await?;
    Ok(())
}

async fn s3_read_bam_header(_event: Value, _ctx: Context) -> Result<Value, Error> {
    let s3_object = stream_s3_object().await?;
    let output = read_bam_header(s3_object).await?;
    dbg!(&output);
    Ok(json!({ "message": &output }))
}

/// Fetches S3 object
async fn stream_s3_object() -> Result<Bytes, Error> {
    SubscriberBuilder::default()
        .with_env_filter("info")
        .with_span_events(FmtSpan::CLOSE)
        .init();
    let conf = s3::Config::builder()
        .region(Region::new(REGION))
        .build();
    let client = s3::Client::from_conf(conf);

    let resp = client.get_object().bucket(BUCKET).key(KEY).send().await?;
    let data = resp.body.collect().await?;

    return Ok(data.into_bytes());
}

/// Reads BAM S3 object header
async fn read_bam_header(bam_bytes: Bytes) -> Result<Value, Error> {
    let mut s3_obj_buffer = Cursor::new(bam_bytes.to_vec());
    // Rewind buffer Cursor after writing, so that next reader can consume header data...
    s3_obj_buffer.set_position(0);

    // ... and read the header
    let mut reader = bam::Reader::new(s3_obj_buffer);
    let raw_header = reader.read_header()?;
    let header: sam::Header = raw_header.parse()?;
    println!("{}", header);

    Ok(json!({ "header": "TBD: Header to be serialized, see https://github.com/brainstorm/s3-rust-htslib-bam/commit/9e7a2002e3d31ac40c87bdad59a4af371b26518f#commitcomment-48811697",
               "message": "success" }))
}
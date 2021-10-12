extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use bytes::Bytes;
use std::io::{Cursor};

use lambda_runtime::{ Context, Error };
use lambda_http::{ handler, Body, Request, Response, IntoResponse };

use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_sdk_s3 as s3;
use s3::Region;

// use tracing_subscriber::fmt::format::FmtSpan;
// use tracing_subscriber::fmt::SubscriberBuilder;

use noodles::bam;
use noodles::sam;
use crate::sam::header::ParseError;

// Change these to your bucket, key and region
const BUCKET: &str = "umccr-research-dev";
const KEY: &str = "htsget/htsnexus_test_NA12878.bam";
const REGION: &str = "ap-southeast-2";


#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(bam_header_as_json)).await?;
    Ok(())
}

/// Turns header string into a JSON (poorly)
/// see: https://github.com/brainstorm/s3-rust-htslib-bam/commit/9e7a2002e3d31ac40c87bdad59a4af371b26518f#commitcomment-48811697
/// ... and ping me if you want to collaborate in serializing bioinformatics formats into Parquet, Arrow, etc... ;)
async fn bam_header_as_json(req: Request, ctx: Context) -> Result<impl IntoResponse, Error> {
    let header = s3_read_bam_header(req, ctx).await?;

    Ok(Response::builder()
        .status(200)
        .body(Body::from(header.to_string()))
        .expect("Something went wrong reading the BAM file")
    )
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
async fn s3_read_bam_header(_event: Request, _: Context) -> Result<sam::Header, Error> {
    let s3_object = stream_s3_object().await?;
    Ok(read_bam_header(s3_object).await?)
}
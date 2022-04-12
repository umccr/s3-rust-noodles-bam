import { Duration, Stack, StackProps } from 'aws-cdk-lib';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { Construct } from 'constructs';
import { RustFunction } from 'rust.aws-cdk-lambda';
import { Architecture } from 'aws-cdk-lib/aws-lambda';
import * as apigw from 'aws-cdk-lib/aws-apigateway';


export class ServerlessBioinformaticsStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props);

        const bucket = new s3.Bucket(this, 'umccr-research-dev', {
            // blockPublicAccess: BlockPublicAccess.BLOCK_ALL,
            // publicReadAccess: false,
        });

        let bamLambda = new RustFunction(this, 'bam_header', {
            functionName: 'bam_header',
            memorySize: 128,
            // Increase the max timeout slightly
            timeout: Duration.seconds(10),
            environment: {
                BUCKET_NAME: bucket.bucketName,
            },
            // Useful so library logs show up in CloudWatch
            setupLogging: true,
            // Enable optional features and env variables at build (compile) time.
            //features: ['my-second-feature'],
            // buildEnvironment: {
            //     MY_BUILD_ENV_VAR: 'Testing 123.',
            // },
            architecture: Architecture.ARM_64
        });

        const api = new apigw.LambdaRestApi(this, 's3-get-bam-header', {
            handler: bamLambda,
            proxy: true,
        });
    }
}
import { Duration, Stack, StackProps } from 'aws-cdk-lib';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { BlockPublicAccess } from 'aws-cdk-lib/aws-s3';
import { Construct } from 'constructs';
import { RustFunction } from 'rust.aws-cdk-lambda';

export class ServerlessBioinformaticsStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props);

        const bucket = new s3.Bucket(this, 'umccr-htsget', {
            blockPublicAccess: BlockPublicAccess.BLOCK_ALL,
            publicReadAccess: false,
        });

        let myLambda = new RustFunction(this, 'HtsGet', {
            functionName: 'htsget-rs',
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
        });

        bucket.grantReadWrite(myLambda);
    }
}

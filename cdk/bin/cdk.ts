#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { GrpcEcsServiceStack } from '../lib/grpc-ecs-service-stack';

const app = new cdk.App();

new GrpcEcsServiceStack(app, 'GrpcEcsServiceStack', {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: process.env.CDK_DEFAULT_REGION,
  }
});

import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as ecs from 'aws-cdk-lib/aws-ecs';
import * as elbv2 from 'aws-cdk-lib/aws-elasticloadbalancingv2';
import * as route53 from 'aws-cdk-lib/aws-route53';
import * as targets from 'aws-cdk-lib/aws-route53-targets';
import * as acm from 'aws-cdk-lib/aws-certificatemanager';
import * as ecr from 'aws-cdk-lib/aws-ecr';
import * as logs from 'aws-cdk-lib/aws-logs';
import * as dotenv from 'dotenv';

dotenv.config();
export class GrpcEcsServiceStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);
    const acmARN = process.env.ACM_ARN || null
    if (acmARN === null) {
      throw new Error('ACM_ARN is required')
    }

    const ecrRepository = ecr.Repository.fromRepositoryName(this, 'ECRRepository', 'rust-grpc');

    const vpc = new ec2.Vpc(this, 'ServiceVPC', {
      maxAzs: 2,
      ipAddresses: ec2.IpAddresses.cidr('10.0.0.0/20'),
    });

    const cluster = new ecs.Cluster(this, 'ServiceCluster', {
      vpc: vpc,
      clusterName: 'grpc-service-cluster',
    });

    const logGroup = new logs.LogGroup(this, 'LogGroup', {
      logGroupName: 'grpc-service-log-group',
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    const certificate = acm.Certificate.fromCertificateArn(this, 'Certificate', acmARN);

    const taskDefinition = new ecs.FargateTaskDefinition(this, 'TaskDef', {
      memoryLimitMiB: 512,
      cpu: 256,
    });

    const container = taskDefinition.addContainer('GrpcServiceContainer', {
      containerName: 'grpc-service-container',
      image: ecs.ContainerImage.fromEcrRepository(ecrRepository, 'v0.2.1'),
      logging: new ecs.AwsLogDriver({
        logGroup: logGroup,
        streamPrefix: 'grpc-service-container',
      }),
      healthCheck: {
        command: ["CMD-SHELL", "grpc-health-probe -addr localhost:50051 -connect-timeout 1s -rpc-timeout 1s || exit 1"],
        interval: cdk.Duration.seconds(30),
        retries: 3,
        startPeriod: cdk.Duration.seconds(5),
        timeout: cdk.Duration.seconds(5),
      },
      portMappings: [
        {
          containerPort: 50051,
          protocol: ecs.Protocol.TCP,
        },
      ]
    });



    const service = new ecs.FargateService(this, 'GrpcService', {
      cluster,
      taskDefinition,
      desiredCount: 1,
      assignPublicIp: true,
      enableExecuteCommand: true,
      serviceName: 'grpc-service',
    });

    const lb = new elbv2.ApplicationLoadBalancer(this, 'LB', {
      vpc,
      internetFacing: true,
      loadBalancerName: 'grpc-service-lb',
      //http2Enabled: true
    });

    const listener = lb.addListener('Listener', {
      port: 443,
      protocol: elbv2.ApplicationProtocol.HTTPS,
      certificates: [certificate],
    });

    const targetGroup = new elbv2.ApplicationTargetGroup(this, 'GrpcTargetGroup', {
      vpc,
      protocol: elbv2.ApplicationProtocol.HTTP,
      protocolVersion: elbv2.ApplicationProtocolVersion.GRPC,
      port: 50051,
      targets: [service],
      targetType: elbv2.TargetType.IP,
      healthCheck: {
        enabled: true,
        interval: cdk.Duration.seconds(30),
        timeout: cdk.Duration.seconds(5),
        healthyThresholdCount: 2,
        unhealthyThresholdCount: 2,
        // GRPC health check configuration (this depends on your grpc-health-probe setup)
        protocol: elbv2.Protocol.HTTP,
        path: '/grpc.health.v1.Health/Check',
        healthyGrpcCodes: '0-99',
      },
      targetGroupName: 'grpc-target-group',
    });

    listener.addTargetGroups('GrpcTargetGroupAttachment', {
      targetGroups: [targetGroup],
    });

    const hostedZone = route53.HostedZone.fromLookup(this, 'HostedZone', {
      domainName: 'cristallum.io',
    });

    new route53.ARecord(this, 'AliasRecord', {
      zone: hostedZone,
      recordName: 'grpc.cristallum.io',
      target: route53.RecordTarget.fromAlias(
        new targets.LoadBalancerTarget(lb)
      ),
    });

    new cdk.CfnOutput(this, 'LoadBalancerDNS', {
      value: lb.loadBalancerDnsName,
    });
  }
}

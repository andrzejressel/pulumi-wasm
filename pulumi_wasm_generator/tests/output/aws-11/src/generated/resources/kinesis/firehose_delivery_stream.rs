/// Provides a Kinesis Firehose Delivery Stream resource. Amazon Kinesis Firehose is a fully managed, elastic service to easily deliver real-time data streams to destinations such as Amazon S3 , Amazon Redshift and Snowflake.
///
/// For more details, see the [Amazon Kinesis Firehose Documentation](https://aws.amazon.com/documentation/firehose/).
///
/// ## Example Usage
///
/// ### Extended S3 Destination
///
/// ```yaml
/// resources:
///   extendedS3Stream:
///     type: aws:kinesis:FirehoseDeliveryStream
///     name: extended_s3_stream
///     properties:
///       name: kinesis-firehose-extended-s3-test-stream
///       destination: extended_s3
///       extendedS3Configuration:
///         roleArn: ${firehoseRole.arn}
///         bucketArn: ${bucket.arn}
///         processingConfiguration:
///           enabled: 'true'
///           processors:
///             - type: Lambda
///               parameters:
///                 - parameterName: LambdaArn
///                   parameterValue: ${lambdaProcessor.arn}:$LATEST
///   bucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: tf-test-bucket
///   bucketAcl:
///     type: aws:s3:BucketAclV2
///     name: bucket_acl
///     properties:
///       bucket: ${bucket.id}
///       acl: private
///   firehoseRole:
///     type: aws:iam:Role
///     name: firehose_role
///     properties:
///       name: firehose_test_role
///       assumeRolePolicy: ${firehoseAssumeRole.json}
///   lambdaIam:
///     type: aws:iam:Role
///     name: lambda_iam
///     properties:
///       name: lambda_iam
///       assumeRolePolicy: ${lambdaAssumeRole.json}
///   lambdaProcessor:
///     type: aws:lambda:Function
///     name: lambda_processor
///     properties:
///       code:
///         fn::FileArchive: lambda.zip
///       name: firehose_lambda_processor
///       role: ${lambdaIam.arn}
///       handler: exports.handler
///       runtime: nodejs20.x
/// variables:
///   firehoseAssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - firehose.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   lambdaAssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - lambda.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ### Extended S3 Destination with dynamic partitioning
///
/// These examples use built-in Firehose functionality, rather than requiring a lambda.
///
/// ```yaml
/// resources:
///   extendedS3Stream:
///     type: aws:kinesis:FirehoseDeliveryStream
///     name: extended_s3_stream
///     properties:
///       name: kinesis-firehose-extended-s3-test-stream
///       destination: extended_s3
///       extendedS3Configuration:
///         roleArn: ${firehoseRole.arn}
///         bucketArn: ${bucket.arn}
///         bufferingSize: 64
///         dynamicPartitioningConfiguration:
///           enabled: 'true'
///         prefix: data/customer_id=!{partitionKeyFromQuery:customer_id}/year=!{timestamp:yyyy}/month=!{timestamp:MM}/day=!{timestamp:dd}/hour=!{timestamp:HH}/
///         errorOutputPrefix: errors/year=!{timestamp:yyyy}/month=!{timestamp:MM}/day=!{timestamp:dd}/hour=!{timestamp:HH}/!{firehose:error-output-type}/
///         processingConfiguration:
///           enabled: 'true'
///           processors:
///             - type: RecordDeAggregation
///               parameters:
///                 - parameterName: SubRecordType
///                   parameterValue: JSON
///             - type: AppendDelimiterToRecord
///             - type: MetadataExtraction
///               parameters:
///                 - parameterName: JsonParsingEngine
///                   parameterValue: JQ-1.6
///                 - parameterName: MetadataExtractionQuery
///                   parameterValue: '{customer_id:.customer_id}'
/// ```
///
/// Multiple Dynamic Partitioning Keys (maximum of 50) can be added by comma separating the `parameter_value`.
///
/// The following example adds the Dynamic Partitioning Keys: `store_id` and `customer_id` to the S3 prefix.
///
/// ```yaml
/// resources:
///   extendedS3Stream:
///     type: aws:kinesis:FirehoseDeliveryStream
///     name: extended_s3_stream
///     properties:
///       name: kinesis-firehose-extended-s3-test-stream
///       destination: extended_s3
///       extendedS3Configuration:
///         roleArn: ${firehoseRole.arn}
///         bucketArn: ${bucket.arn}
///         bufferingSize: 64
///         dynamicPartitioningConfiguration:
///           enabled: 'true'
///         prefix: data/store_id=!{partitionKeyFromQuery:store_id}/customer_id=!{partitionKeyFromQuery:customer_id}/year=!{timestamp:yyyy}/month=!{timestamp:MM}/day=!{timestamp:dd}/hour=!{timestamp:HH}/
///         errorOutputPrefix: errors/year=!{timestamp:yyyy}/month=!{timestamp:MM}/day=!{timestamp:dd}/hour=!{timestamp:HH}/!{firehose:error-output-type}/
///         processingConfiguration:
///           enabled: 'true'
///           processors:
///             - type: MetadataExtraction
///               parameters:
///                 - parameterName: JsonParsingEngine
///                   parameterValue: JQ-1.6
///                 - parameterName: MetadataExtractionQuery
///                   parameterValue: '{store_id:.store_id,customer_id:.customer_id}'
/// ```
///
/// ### Redshift Destination
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testCluster = cluster::create(
///         "testCluster",
///         ClusterArgs::builder()
///             .cluster_identifier("tf-redshift-cluster")
///             .cluster_type("single-node")
///             .database_name("test")
///             .master_password("T3stPass")
///             .master_username("testuser")
///             .node_type("dc1.large")
///             .build_struct(),
///     );
///     let testStream = firehose_delivery_stream::create(
///         "testStream",
///         FirehoseDeliveryStreamArgs::builder()
///             .destination("redshift")
///             .name("kinesis-firehose-test-stream")
///             .redshift_configuration(
///                 FirehoseDeliveryStreamRedshiftConfiguration::builder()
///                     .clusterJdbcurl(
///                         "jdbc:redshift://${testCluster.endpoint}/${testCluster.databaseName}",
///                     )
///                     .copyOptions("delimiter '|'")
///                     .dataTableColumns("test-col")
///                     .dataTableName("test-table")
///                     .password("T3stPass")
///                     .roleArn("${firehoseRole.arn}")
///                     .s3BackupConfiguration(
///                         FirehoseDeliveryStreamRedshiftConfigurationS3BackupConfiguration::builder()
///                             .bucketArn("${bucket.arn}")
///                             .bufferingInterval(300)
///                             .bufferingSize(15)
///                             .compressionFormat("GZIP")
///                             .roleArn("${firehoseRole.arn}")
///                             .build_struct(),
///                     )
///                     .s3BackupMode("Enabled")
///                     .s3Configuration(
///                         FirehoseDeliveryStreamRedshiftConfigurationS3Configuration::builder()
///                             .bucketArn("${bucket.arn}")
///                             .bufferingInterval(400)
///                             .bufferingSize(10)
///                             .compressionFormat("GZIP")
///                             .roleArn("${firehoseRole.arn}")
///                             .build_struct(),
///                     )
///                     .username("testuser")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Elasticsearch Destination
///
/// ```yaml
/// resources:
///   testCluster:
///     type: aws:elasticsearch:Domain
///     name: test_cluster
///     properties:
///       domainName: firehose-es-test
///   testStream:
///     type: aws:kinesis:FirehoseDeliveryStream
///     name: test_stream
///     properties:
///       name: kinesis-firehose-test-stream
///       destination: elasticsearch
///       elasticsearchConfiguration:
///         domainArn: ${testCluster.arn}
///         roleArn: ${firehoseRole.arn}
///         indexName: test
///         typeName: test
///         s3Configuration:
///           roleArn: ${firehoseRole.arn}
///           bucketArn: ${bucket.arn}
///           bufferingSize: 10
///           bufferingInterval: 400
///           compressionFormat: GZIP
///         processingConfiguration:
///           enabled: 'true'
///           processors:
///             - type: Lambda
///               parameters:
///                 - parameterName: LambdaArn
///                   parameterValue: ${lambdaProcessor.arn}:$LATEST
/// ```
///
/// ### Elasticsearch Destination With VPC
///
/// ```yaml
/// resources:
///   testCluster:
///     type: aws:elasticsearch:Domain
///     name: test_cluster
///     properties:
///       domainName: es-test
///       clusterConfig:
///         instanceCount: 2
///         zoneAwarenessEnabled: true
///         instanceType: t2.small.elasticsearch
///       ebsOptions:
///         ebsEnabled: true
///         volumeSize: 10
///       vpcOptions:
///         securityGroupIds:
///           - ${first.id}
///         subnetIds:
///           - ${firstAwsSubnet.id}
///           - ${second.id}
///   firehose-elasticsearchRolePolicy:
///     type: aws:iam:RolePolicy
///     name: firehose-elasticsearch
///     properties:
///       name: elasticsearch
///       role: ${firehose.id}
///       policy: ${["firehose-elasticsearch"].json}
///   test:
///     type: aws:kinesis:FirehoseDeliveryStream
///     properties:
///       name: kinesis-firehose-es
///       destination: elasticsearch
///       elasticsearchConfiguration:
///         domainArn: ${testCluster.arn}
///         roleArn: ${firehose.arn}
///         indexName: test
///         typeName: test
///         s3Configuration:
///           roleArn: ${firehose.arn}
///           bucketArn: ${bucket.arn}
///         vpcConfig:
///           subnetIds:
///             - ${firstAwsSubnet.id}
///             - ${second.id}
///           securityGroupIds:
///             - ${first.id}
///           roleArn: ${firehose.arn}
///     options:
///       dependsOn:
///         - ${["firehose-elasticsearchRolePolicy"]}
/// variables:
///   firehose-elasticsearch:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - es:*
///             resources:
///               - ${testCluster.arn}
///               - ${testCluster.arn}/*
///           - effect: Allow
///             actions:
///               - ec2:DescribeVpcs
///               - ec2:DescribeVpcAttribute
///               - ec2:DescribeSubnets
///               - ec2:DescribeSecurityGroups
///               - ec2:DescribeNetworkInterfaces
///               - ec2:CreateNetworkInterface
///               - ec2:CreateNetworkInterfacePermission
///               - ec2:DeleteNetworkInterface
///             resources:
///               - '*'
/// ```
///
/// ### OpenSearch Destination
///
/// ```yaml
/// resources:
///   testCluster:
///     type: aws:opensearch:Domain
///     name: test_cluster
///     properties:
///       domainName: firehose-os-test
///   testStream:
///     type: aws:kinesis:FirehoseDeliveryStream
///     name: test_stream
///     properties:
///       name: kinesis-firehose-test-stream
///       destination: opensearch
///       opensearchConfiguration:
///         domainArn: ${testCluster.arn}
///         roleArn: ${firehoseRole.arn}
///         indexName: test
///         s3Configuration:
///           roleArn: ${firehoseRole.arn}
///           bucketArn: ${bucket.arn}
///           bufferingSize: 10
///           bufferingInterval: 400
///           compressionFormat: GZIP
///         processingConfiguration:
///           enabled: 'true'
///           processors:
///             - type: Lambda
///               parameters:
///                 - parameterName: LambdaArn
///                   parameterValue: ${lambdaProcessor.arn}:$LATEST
/// ```
///
/// ### OpenSearch Destination With VPC
///
/// ```yaml
/// resources:
///   testCluster:
///     type: aws:opensearch:Domain
///     name: test_cluster
///     properties:
///       domainName: es-test
///       clusterConfig:
///         instanceCount: 2
///         zoneAwarenessEnabled: true
///         instanceType: m4.large.search
///       ebsOptions:
///         ebsEnabled: true
///         volumeSize: 10
///       vpcOptions:
///         securityGroupIds:
///           - ${first.id}
///         subnetIds:
///           - ${firstAwsSubnet.id}
///           - ${second.id}
///   firehose-opensearch:
///     type: aws:iam:RolePolicy
///     properties:
///       name: opensearch
///       role: ${firehose.id}
///       policy: |
///         {
///           "Version": "2012-10-17",
///           "Statement": [
///             {
///               "Effect": "Allow",
///               "Action": [
///                 "es:*"
///               ],
///               "Resource": [
///                 "${testCluster.arn}",
///                 "${testCluster.arn}/*"
///               ]
///                 },
///                 {
///                   "Effect": "Allow",
///                   "Action": [
///                     "ec2:DescribeVpcs",
///                     "ec2:DescribeVpcAttribute",
///                     "ec2:DescribeSubnets",
///                     "ec2:DescribeSecurityGroups",
///                     "ec2:DescribeNetworkInterfaces",
///                     "ec2:CreateNetworkInterface",
///                     "ec2:CreateNetworkInterfacePermission",
///                     "ec2:DeleteNetworkInterface"
///                   ],
///                   "Resource": [
///                     "*"
///                   ]
///                 }
///           ]
///         }
///   test:
///     type: aws:kinesis:FirehoseDeliveryStream
///     properties:
///       name: pulumi-kinesis-firehose-os
///       destination: opensearch
///       opensearchConfiguration:
///         domainArn: ${testCluster.arn}
///         roleArn: ${firehose.arn}
///         indexName: test
///         s3Configuration:
///           roleArn: ${firehose.arn}
///           bucketArn: ${bucket.arn}
///         vpcConfig:
///           subnetIds:
///             - ${firstAwsSubnet.id}
///             - ${second.id}
///           securityGroupIds:
///             - ${first.id}
///           roleArn: ${firehose.arn}
///     options:
///       dependsOn:
///         - ${["firehose-opensearch"]}
/// ```
///
/// ### OpenSearch Serverless Destination
///
/// ```yaml
/// resources:
///   testCollection:
///     type: aws:opensearch:ServerlessCollection
///     name: test_collection
///     properties:
///       name: firehose-osserverless-test
///   testStream:
///     type: aws:kinesis:FirehoseDeliveryStream
///     name: test_stream
///     properties:
///       name: kinesis-firehose-test-stream
///       destination: opensearchserverless
///       opensearchserverlessConfiguration:
///         collectionEndpoint: ${testCollection.collectionEndpoint}
///         roleArn: ${firehoseRole.arn}
///         indexName: test
///         s3Configuration:
///           roleArn: ${firehoseRole.arn}
///           bucketArn: ${bucket.arn}
///           bufferingSize: 10
///           bufferingInterval: 400
///           compressionFormat: GZIP
///         processingConfiguration:
///           enabled: 'true'
///           processors:
///             - type: Lambda
///               parameters:
///                 - parameterName: LambdaArn
///                   parameterValue: ${lambdaProcessor.arn}:$LATEST
/// ```
///
/// ### Iceberg Destination
///
/// ```yaml
/// resources:
///   bucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: test-bucket
///       forceDestroy: true
///   test:
///     type: aws:glue:CatalogDatabase
///     properties:
///       name: test
///   testCatalogTable:
///     type: aws:glue:CatalogTable
///     name: test
///     properties:
///       name: test
///       databaseName: ${test.name}
///       parameters:
///         format: parquet
///       tableType: EXTERNAL_TABLE
///       openTableFormatInput:
///         icebergInput:
///           metadataOperation: CREATE
///           version: 2
///       storageDescriptor:
///         location: s3://${bucket.id}
///         columns:
///           - name: my_column_1
///             type: int
///   testStream:
///     type: aws:kinesis:FirehoseDeliveryStream
///     name: test_stream
///     properties:
///       name: kinesis-firehose-test-stream
///       destination: iceberg
///       icebergConfiguration:
///         roleArn: ${firehoseRole.arn}
///         catalogArn: arn:${currentGetPartition.partition}:glue:${currentGetRegion.name}:${current.accountId}:catalog
///         bufferingSize: 10
///         bufferingInterval: 400
///         s3Configuration:
///           roleArn: ${firehoseRole.arn}
///           bucketArn: ${bucket.arn}
///         destinationTableConfigurations:
///           - databaseName: ${test.name}
///             tableName: ${testCatalogTable.name}
///         processingConfiguration:
///           enabled: 'true'
///           processors:
///             - type: Lambda
///               parameters:
///                 - parameterName: LambdaArn
///                   parameterValue: ${lambdaProcessor.arn}:$LATEST
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   currentGetPartition:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
///   currentGetRegion:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ### Splunk Destination
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testStream = firehose_delivery_stream::create(
///         "testStream",
///         FirehoseDeliveryStreamArgs::builder()
///             .destination("splunk")
///             .name("kinesis-firehose-test-stream")
///             .splunk_configuration(
///                 FirehoseDeliveryStreamSplunkConfiguration::builder()
///                     .hecAcknowledgmentTimeout(600)
///                     .hecEndpoint("https://http-inputs-mydomain.splunkcloud.com:443")
///                     .hecEndpointType("Event")
///                     .hecToken("51D4DA16-C61B-4F5F-8EC7-ED4301342A4A")
///                     .s3BackupMode("FailedEventsOnly")
///                     .s3Configuration(
///                         FirehoseDeliveryStreamSplunkConfigurationS3Configuration::builder()
///                             .bucketArn("${bucket.arn}")
///                             .bufferingInterval(400)
///                             .bufferingSize(10)
///                             .compressionFormat("GZIP")
///                             .roleArn("${firehose.arn}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### HTTP Endpoint (e.g., New Relic) Destination
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testStream = firehose_delivery_stream::create(
///         "testStream",
///         FirehoseDeliveryStreamArgs::builder()
///             .destination("http_endpoint")
///             .http_endpoint_configuration(
///                 FirehoseDeliveryStreamHttpEndpointConfiguration::builder()
///                     .accessKey("my-key")
///                     .bufferingInterval(600)
///                     .bufferingSize(15)
///                     .name("New Relic")
///                     .requestConfiguration(
///                         FirehoseDeliveryStreamHttpEndpointConfigurationRequestConfiguration::builder()
///                             .commonAttributes(
///                                 vec![
///                                     FirehoseDeliveryStreamHttpEndpointConfigurationRequestConfigurationCommonAttribute::builder()
///                                     .name("testname").value("testvalue").build_struct(),
///                                     FirehoseDeliveryStreamHttpEndpointConfigurationRequestConfigurationCommonAttribute::builder()
///                                     .name("testname2").value("testvalue2").build_struct(),
///                                 ],
///                             )
///                             .contentEncoding("GZIP")
///                             .build_struct(),
///                     )
///                     .roleArn("${firehose.arn}")
///                     .s3BackupMode("FailedDataOnly")
///                     .s3Configuration(
///                         FirehoseDeliveryStreamHttpEndpointConfigurationS3Configuration::builder()
///                             .bucketArn("${bucket.arn}")
///                             .bufferingInterval(400)
///                             .bufferingSize(10)
///                             .compressionFormat("GZIP")
///                             .roleArn("${firehose.arn}")
///                             .build_struct(),
///                     )
///                     .url("https://aws-api.newrelic.com/firehose/v1")
///                     .build_struct(),
///             )
///             .name("kinesis-firehose-test-stream")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Snowflake Destination
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let exampleSnowflakeDestination = firehose_delivery_stream::create(
///         "exampleSnowflakeDestination",
///         FirehoseDeliveryStreamArgs::builder()
///             .destination("snowflake")
///             .name("example-snowflake-destination")
///             .snowflake_configuration(
///                 FirehoseDeliveryStreamSnowflakeConfiguration::builder()
///                     .accountUrl("https://example.snowflakecomputing.com")
///                     .bufferingInterval(600)
///                     .bufferingSize(15)
///                     .database("example-db")
///                     .privateKey("...")
///                     .roleArn("${firehose.arn}")
///                     .s3Configuration(
///                         FirehoseDeliveryStreamSnowflakeConfigurationS3Configuration::builder()
///                             .bucketArn("${bucket.arn}")
///                             .bufferingInterval(400)
///                             .bufferingSize(10)
///                             .compressionFormat("GZIP")
///                             .roleArn("${firehose.arn}")
///                             .build_struct(),
///                     )
///                     .schema("example-schema")
///                     .table("example-table")
///                     .user("example-usr")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Kinesis Firehose Delivery streams using the stream ARN. For example:
///
/// ```sh
/// $ pulumi import aws:kinesis/firehoseDeliveryStream:FirehoseDeliveryStream foo arn:aws:firehose:us-east-1:XXX:deliverystream/example
/// ```
/// Note: Import does not work for stream destination `s3`. Consider using `extended_s3` since `s3` destination is deprecated.
///
pub mod firehose_delivery_stream {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirehoseDeliveryStreamArgs {
        /// The Amazon Resource Name (ARN) specifying the Stream
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        /// This is the destination to where the data is delivered. The only options are `s3` (Deprecated, use `extended_s3` instead), `extended_s3`, `redshift`, `elasticsearch`, `splunk`, `http_endpoint`, `opensearch`, `opensearchserverless` and `snowflake`.
        #[builder(into)]
        pub destination: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub destination_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration options when `destination` is `elasticsearch`. See `elasticsearch_configuration` block below for details.
        #[builder(into, default)]
        pub elasticsearch_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamElasticsearchConfiguration,
            >,
        >,
        /// Enhanced configuration options for the s3 destination. See `extended_s3_configuration` block below for details.
        #[builder(into, default)]
        pub extended_s3_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3Configuration,
            >,
        >,
        /// Configuration options when `destination` is `http_endpoint`. Requires the user to also specify an `s3_configuration` block.  See `http_endpoint_configuration` block below for details.
        #[builder(into, default)]
        pub http_endpoint_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamHttpEndpointConfiguration,
            >,
        >,
        /// Configuration options when `destination` is `iceberg`. See `iceberg_configuration` block below for details.
        #[builder(into, default)]
        pub iceberg_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfiguration,
            >,
        >,
        /// The stream and role Amazon Resource Names (ARNs) for a Kinesis data stream used as the source for a delivery stream. See `kinesis_source_configuration` block below for details.
        #[builder(into, default)]
        pub kinesis_source_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamKinesisSourceConfiguration,
            >,
        >,
        /// The configuration for the Amazon MSK cluster to be used as the source for a delivery stream. See `msk_source_configuration` block below for details.
        #[builder(into, default)]
        pub msk_source_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamMskSourceConfiguration,
            >,
        >,
        /// A name to identify the stream. This is unique to the AWS account and region the Stream is created in. When using for WAF logging, name must be prefixed with `aws-waf-logs-`. See [AWS Documentation](https://docs.aws.amazon.com/waf/latest/developerguide/waf-policies.html#waf-policies-logging-config) for more details.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration options when `destination` is `opensearch`. See `opensearch_configuration` block below for details.
        #[builder(into, default)]
        pub opensearch_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamOpensearchConfiguration,
            >,
        >,
        /// Configuration options when `destination` is `opensearchserverless`. See `opensearchserverless_configuration` block below for details.
        #[builder(into, default)]
        pub opensearchserverless_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfiguration,
            >,
        >,
        /// Configuration options when `destination` is `redshift`. Requires the user to also specify an `s3_configuration` block. See `redshift_configuration` block below for details.
        #[builder(into, default)]
        pub redshift_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfiguration,
            >,
        >,
        /// Encrypt at rest options. See `server_side_encryption` block below for details.
        ///
        /// **NOTE:** Server-side encryption should not be enabled when a kinesis stream is configured as the source of the firehose delivery stream.
        #[builder(into, default)]
        pub server_side_encryption: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamServerSideEncryption,
            >,
        >,
        /// Configuration options when `destination` is `snowflake`. See `snowflake_configuration` block below for details.
        #[builder(into, default)]
        pub snowflake_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfiguration,
            >,
        >,
        /// Configuration options when `destination` is `splunk`. See `splunk_configuration` block below for details.
        #[builder(into, default)]
        pub splunk_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamSplunkConfiguration,
            >,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub version_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FirehoseDeliveryStreamResult {
        /// The Amazon Resource Name (ARN) specifying the Stream
        pub arn: pulumi_wasm_rust::Output<String>,
        /// This is the destination to where the data is delivered. The only options are `s3` (Deprecated, use `extended_s3` instead), `extended_s3`, `redshift`, `elasticsearch`, `splunk`, `http_endpoint`, `opensearch`, `opensearchserverless` and `snowflake`.
        pub destination: pulumi_wasm_rust::Output<String>,
        pub destination_id: pulumi_wasm_rust::Output<String>,
        /// Configuration options when `destination` is `elasticsearch`. See `elasticsearch_configuration` block below for details.
        pub elasticsearch_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamElasticsearchConfiguration,
            >,
        >,
        /// Enhanced configuration options for the s3 destination. See `extended_s3_configuration` block below for details.
        pub extended_s3_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3Configuration,
            >,
        >,
        /// Configuration options when `destination` is `http_endpoint`. Requires the user to also specify an `s3_configuration` block.  See `http_endpoint_configuration` block below for details.
        pub http_endpoint_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamHttpEndpointConfiguration,
            >,
        >,
        /// Configuration options when `destination` is `iceberg`. See `iceberg_configuration` block below for details.
        pub iceberg_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamIcebergConfiguration,
            >,
        >,
        /// The stream and role Amazon Resource Names (ARNs) for a Kinesis data stream used as the source for a delivery stream. See `kinesis_source_configuration` block below for details.
        pub kinesis_source_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamKinesisSourceConfiguration,
            >,
        >,
        /// The configuration for the Amazon MSK cluster to be used as the source for a delivery stream. See `msk_source_configuration` block below for details.
        pub msk_source_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamMskSourceConfiguration,
            >,
        >,
        /// A name to identify the stream. This is unique to the AWS account and region the Stream is created in. When using for WAF logging, name must be prefixed with `aws-waf-logs-`. See [AWS Documentation](https://docs.aws.amazon.com/waf/latest/developerguide/waf-policies.html#waf-policies-logging-config) for more details.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Configuration options when `destination` is `opensearch`. See `opensearch_configuration` block below for details.
        pub opensearch_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamOpensearchConfiguration,
            >,
        >,
        /// Configuration options when `destination` is `opensearchserverless`. See `opensearchserverless_configuration` block below for details.
        pub opensearchserverless_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamOpensearchserverlessConfiguration,
            >,
        >,
        /// Configuration options when `destination` is `redshift`. Requires the user to also specify an `s3_configuration` block. See `redshift_configuration` block below for details.
        pub redshift_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfiguration,
            >,
        >,
        /// Encrypt at rest options. See `server_side_encryption` block below for details.
        ///
        /// **NOTE:** Server-side encryption should not be enabled when a kinesis stream is configured as the source of the firehose delivery stream.
        pub server_side_encryption: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamServerSideEncryption,
            >,
        >,
        /// Configuration options when `destination` is `snowflake`. See `snowflake_configuration` block below for details.
        pub snowflake_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfiguration,
            >,
        >,
        /// Configuration options when `destination` is `splunk`. See `splunk_configuration` block below for details.
        pub splunk_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::kinesis::FirehoseDeliveryStreamSplunkConfiguration,
            >,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub version_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FirehoseDeliveryStreamArgs,
    ) -> FirehoseDeliveryStreamResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let destination_binding = args.destination.get_inner();
        let destination_id_binding = args.destination_id.get_inner();
        let elasticsearch_configuration_binding = args
            .elasticsearch_configuration
            .get_inner();
        let extended_s3_configuration_binding = args
            .extended_s3_configuration
            .get_inner();
        let http_endpoint_configuration_binding = args
            .http_endpoint_configuration
            .get_inner();
        let iceberg_configuration_binding = args.iceberg_configuration.get_inner();
        let kinesis_source_configuration_binding = args
            .kinesis_source_configuration
            .get_inner();
        let msk_source_configuration_binding = args.msk_source_configuration.get_inner();
        let name_binding = args.name.get_inner();
        let opensearch_configuration_binding = args.opensearch_configuration.get_inner();
        let opensearchserverless_configuration_binding = args
            .opensearchserverless_configuration
            .get_inner();
        let redshift_configuration_binding = args.redshift_configuration.get_inner();
        let server_side_encryption_binding = args.server_side_encryption.get_inner();
        let snowflake_configuration_binding = args.snowflake_configuration.get_inner();
        let splunk_configuration_binding = args.splunk_configuration.get_inner();
        let tags_binding = args.tags.get_inner();
        let version_id_binding = args.version_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kinesis/firehoseDeliveryStream:FirehoseDeliveryStream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "destination".into(),
                    value: &destination_binding,
                },
                register_interface::ObjectField {
                    name: "destinationId".into(),
                    value: &destination_id_binding,
                },
                register_interface::ObjectField {
                    name: "elasticsearchConfiguration".into(),
                    value: &elasticsearch_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "extendedS3Configuration".into(),
                    value: &extended_s3_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "httpEndpointConfiguration".into(),
                    value: &http_endpoint_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "icebergConfiguration".into(),
                    value: &iceberg_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "kinesisSourceConfiguration".into(),
                    value: &kinesis_source_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "mskSourceConfiguration".into(),
                    value: &msk_source_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "opensearchConfiguration".into(),
                    value: &opensearch_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "opensearchserverlessConfiguration".into(),
                    value: &opensearchserverless_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "redshiftConfiguration".into(),
                    value: &redshift_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "serverSideEncryption".into(),
                    value: &server_side_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "snowflakeConfiguration".into(),
                    value: &snowflake_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "splunkConfiguration".into(),
                    value: &splunk_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "versionId".into(),
                    value: &version_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "destination".into(),
                },
                register_interface::ResultField {
                    name: "destinationId".into(),
                },
                register_interface::ResultField {
                    name: "elasticsearchConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "extendedS3Configuration".into(),
                },
                register_interface::ResultField {
                    name: "httpEndpointConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "icebergConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "kinesisSourceConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "mskSourceConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "opensearchConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "opensearchserverlessConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "redshiftConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "serverSideEncryption".into(),
                },
                register_interface::ResultField {
                    name: "snowflakeConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "splunkConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "versionId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FirehoseDeliveryStreamResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destination").unwrap(),
            ),
            destination_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationId").unwrap(),
            ),
            elasticsearch_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticsearchConfiguration").unwrap(),
            ),
            extended_s3_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extendedS3Configuration").unwrap(),
            ),
            http_endpoint_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpEndpointConfiguration").unwrap(),
            ),
            iceberg_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("icebergConfiguration").unwrap(),
            ),
            kinesis_source_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kinesisSourceConfiguration").unwrap(),
            ),
            msk_source_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mskSourceConfiguration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            opensearch_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("opensearchConfiguration").unwrap(),
            ),
            opensearchserverless_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("opensearchserverlessConfiguration").unwrap(),
            ),
            redshift_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("redshiftConfiguration").unwrap(),
            ),
            server_side_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverSideEncryption").unwrap(),
            ),
            snowflake_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snowflakeConfiguration").unwrap(),
            ),
            splunk_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("splunkConfiguration").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionId").unwrap(),
            ),
        }
    }
}

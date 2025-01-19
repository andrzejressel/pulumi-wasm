/// Provides a VPC/Subnet/ENI/Transit Gateway/Transit Gateway Attachment Flow Log to capture IP traffic for a specific network
/// interface, subnet, or VPC. Logs are sent to a CloudWatch Log Group, a S3 Bucket, or Amazon Kinesis Data Firehose
///
/// ## Example Usage
///
/// ### CloudWatch Logging
///
/// ```yaml
/// resources:
///   exampleFlowLog:
///     type: aws:ec2:FlowLog
///     name: example
///     properties:
///       iamRoleArn: ${exampleRole.arn}
///       logDestination: ${exampleLogGroup.arn}
///       trafficType: ALL
///       vpcId: ${exampleAwsVpc.id}
///   exampleLogGroup:
///     type: aws:cloudwatch:LogGroup
///     name: example
///     properties:
///       name: example
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example
///       assumeRolePolicy: ${assumeRole.json}
///   exampleRolePolicy:
///     type: aws:iam:RolePolicy
///     name: example
///     properties:
///       name: example
///       role: ${exampleRole.id}
///       policy: ${example.json}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - vpc-flow-logs.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - logs:CreateLogGroup
///               - logs:CreateLogStream
///               - logs:PutLogEvents
///               - logs:DescribeLogGroups
///               - logs:DescribeLogStreams
///             resources:
///               - '*'
/// ```
///
/// ### Amazon Kinesis Data Firehose logging
///
/// ```yaml
/// resources:
///   exampleFlowLog:
///     type: aws:ec2:FlowLog
///     name: example
///     properties:
///       logDestination: ${exampleFirehoseDeliveryStream.arn}
///       logDestinationType: kinesis-data-firehose
///       trafficType: ALL
///       vpcId: ${exampleAwsVpc.id}
///   exampleFirehoseDeliveryStream:
///     type: aws:kinesis:FirehoseDeliveryStream
///     name: example
///     properties:
///       name: kinesis_firehose_test
///       destination: extended_s3
///       extendedS3Configuration:
///         roleArn: ${exampleRole.arn}
///         bucketArn: ${exampleBucketV2.arn}
///       tags:
///         LogDeliveryEnabled: 'true'
///   exampleBucketV2:
///     type: aws:s3:BucketV2
///     name: example
///     properties:
///       bucket: example
///   exampleBucketAclV2:
///     type: aws:s3:BucketAclV2
///     name: example
///     properties:
///       bucket: ${exampleBucketV2.id}
///       acl: private
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: firehose_test_role
///       assumeRolePolicy: ${assumeRole.json}
///   exampleRolePolicy:
///     type: aws:iam:RolePolicy
///     name: example
///     properties:
///       name: test
///       role: ${exampleRole.id}
///       policy: ${example.json}
/// variables:
///   assumeRole:
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
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         effect: Allow
///         actions:
///           - logs:CreateLogDelivery
///           - logs:DeleteLogDelivery
///           - logs:ListLogDeliveries
///           - logs:GetLogDelivery
///           - firehose:TagDeliveryStream
///         resources:
///           - '*'
/// ```
///
/// ### S3 Logging
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = flow_log::create(
///         "example",
///         FlowLogArgs::builder()
///             .log_destination("${exampleBucketV2.arn}")
///             .log_destination_type("s3")
///             .traffic_type("ALL")
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
///     let exampleBucketV2 = bucket_v_2::create(
///         "exampleBucketV2",
///         BucketV2Args::builder().bucket("example").build_struct(),
///     );
/// }
/// ```
///
/// ### S3 Logging in Apache Parquet format with per-hour partitions
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = flow_log::create(
///         "example",
///         FlowLogArgs::builder()
///             .destination_options(
///                 FlowLogDestinationOptions::builder()
///                     .fileFormat("parquet")
///                     .perHourPartition(true)
///                     .build_struct(),
///             )
///             .log_destination("${exampleBucketV2.arn}")
///             .log_destination_type("s3")
///             .traffic_type("ALL")
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
///     let exampleBucketV2 = bucket_v_2::create(
///         "exampleBucketV2",
///         BucketV2Args::builder().bucket("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Flow Logs using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/flowLog:FlowLog test_flow_log fl-1a2b3c4d
/// ```
pub mod flow_log {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlowLogArgs {
        /// ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.
        #[builder(into, default)]
        pub deliver_cross_account_role: pulumi_wasm_rust::Output<Option<String>>,
        /// Describes the destination options for a flow log. More details below.
        #[builder(into, default)]
        pub destination_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::FlowLogDestinationOptions>,
        >,
        /// Elastic Network Interface ID to attach to
        #[builder(into, default)]
        pub eni_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN for the IAM role that's used to post flow logs to a CloudWatch Logs log group
        #[builder(into, default)]
        pub iam_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the logging destination. Either `log_destination` or `log_group_name` must be set.
        #[builder(into, default)]
        pub log_destination: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of the logging destination. Valid values: `cloud-watch-logs`, `s3`, `kinesis-data-firehose`. Default: `cloud-watch-logs`.
        #[builder(into, default)]
        pub log_destination_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The fields to include in the flow log record. Accepted format example: `"$${interface-id} $${srcaddr} $${dstaddr} $${srcport} $${dstport}"`.
        #[builder(into, default)]
        pub log_format: pulumi_wasm_rust::Output<Option<String>>,
        /// **Deprecated:** Use `log_destination` instead. The name of the CloudWatch log group. Either `log_group_name` or `log_destination` must be set.
        #[builder(into, default)]
        pub log_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum interval of time
        /// during which a flow of packets is captured and aggregated into a flow
        /// log record. Valid Values: `60` seconds (1 minute) or `600` seconds (10
        /// minutes). Default: `600`. When `transit_gateway_id` or `transit_gateway_attachment_id` is specified, `max_aggregation_interval` *must* be 60 seconds (1 minute).
        #[builder(into, default)]
        pub max_aggregation_interval: pulumi_wasm_rust::Output<Option<i32>>,
        /// Subnet ID to attach to
        #[builder(into, default)]
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of traffic to capture. Valid values: `ACCEPT`,`REJECT`, `ALL`.
        #[builder(into, default)]
        pub traffic_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Transit Gateway Attachment ID to attach to
        #[builder(into, default)]
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Transit Gateway ID to attach to
        #[builder(into, default)]
        pub transit_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// VPC ID to attach to
        #[builder(into, default)]
        pub vpc_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FlowLogResult {
        /// The ARN of the Flow Log.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.
        pub deliver_cross_account_role: pulumi_wasm_rust::Output<Option<String>>,
        /// Describes the destination options for a flow log. More details below.
        pub destination_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::FlowLogDestinationOptions>,
        >,
        /// Elastic Network Interface ID to attach to
        pub eni_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN for the IAM role that's used to post flow logs to a CloudWatch Logs log group
        pub iam_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the logging destination. Either `log_destination` or `log_group_name` must be set.
        pub log_destination: pulumi_wasm_rust::Output<String>,
        /// The type of the logging destination. Valid values: `cloud-watch-logs`, `s3`, `kinesis-data-firehose`. Default: `cloud-watch-logs`.
        pub log_destination_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The fields to include in the flow log record. Accepted format example: `"$${interface-id} $${srcaddr} $${dstaddr} $${srcport} $${dstport}"`.
        pub log_format: pulumi_wasm_rust::Output<String>,
        /// **Deprecated:** Use `log_destination` instead. The name of the CloudWatch log group. Either `log_group_name` or `log_destination` must be set.
        pub log_group_name: pulumi_wasm_rust::Output<String>,
        /// The maximum interval of time
        /// during which a flow of packets is captured and aggregated into a flow
        /// log record. Valid Values: `60` seconds (1 minute) or `600` seconds (10
        /// minutes). Default: `600`. When `transit_gateway_id` or `transit_gateway_attachment_id` is specified, `max_aggregation_interval` *must* be 60 seconds (1 minute).
        pub max_aggregation_interval: pulumi_wasm_rust::Output<Option<i32>>,
        /// Subnet ID to attach to
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of traffic to capture. Valid values: `ACCEPT`,`REJECT`, `ALL`.
        pub traffic_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Transit Gateway Attachment ID to attach to
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Transit Gateway ID to attach to
        pub transit_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// VPC ID to attach to
        pub vpc_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FlowLogArgs) -> FlowLogResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deliver_cross_account_role_binding = args
            .deliver_cross_account_role
            .get_inner();
        let destination_options_binding = args.destination_options.get_inner();
        let eni_id_binding = args.eni_id.get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_inner();
        let log_destination_binding = args.log_destination.get_inner();
        let log_destination_type_binding = args.log_destination_type.get_inner();
        let log_format_binding = args.log_format.get_inner();
        let log_group_name_binding = args.log_group_name.get_inner();
        let max_aggregation_interval_binding = args.max_aggregation_interval.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let traffic_type_binding = args.traffic_type.get_inner();
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_inner();
        let transit_gateway_id_binding = args.transit_gateway_id.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/flowLog:FlowLog".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deliverCrossAccountRole".into(),
                    value: &deliver_cross_account_role_binding,
                },
                register_interface::ObjectField {
                    name: "destinationOptions".into(),
                    value: &destination_options_binding,
                },
                register_interface::ObjectField {
                    name: "eniId".into(),
                    value: &eni_id_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "logDestination".into(),
                    value: &log_destination_binding,
                },
                register_interface::ObjectField {
                    name: "logDestinationType".into(),
                    value: &log_destination_type_binding,
                },
                register_interface::ObjectField {
                    name: "logFormat".into(),
                    value: &log_format_binding,
                },
                register_interface::ObjectField {
                    name: "logGroupName".into(),
                    value: &log_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "maxAggregationInterval".into(),
                    value: &max_aggregation_interval_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trafficType".into(),
                    value: &traffic_type_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "deliverCrossAccountRole".into(),
                },
                register_interface::ResultField {
                    name: "destinationOptions".into(),
                },
                register_interface::ResultField {
                    name: "eniId".into(),
                },
                register_interface::ResultField {
                    name: "iamRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "logDestination".into(),
                },
                register_interface::ResultField {
                    name: "logDestinationType".into(),
                },
                register_interface::ResultField {
                    name: "logFormat".into(),
                },
                register_interface::ResultField {
                    name: "logGroupName".into(),
                },
                register_interface::ResultField {
                    name: "maxAggregationInterval".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "trafficType".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAttachmentId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FlowLogResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            deliver_cross_account_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliverCrossAccountRole").unwrap(),
            ),
            destination_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationOptions").unwrap(),
            ),
            eni_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eniId").unwrap(),
            ),
            iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoleArn").unwrap(),
            ),
            log_destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logDestination").unwrap(),
            ),
            log_destination_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logDestinationType").unwrap(),
            ),
            log_format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logFormat").unwrap(),
            ),
            log_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logGroupName").unwrap(),
            ),
            max_aggregation_interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxAggregationInterval").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            traffic_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficType").unwrap(),
            ),
            transit_gateway_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAttachmentId").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}

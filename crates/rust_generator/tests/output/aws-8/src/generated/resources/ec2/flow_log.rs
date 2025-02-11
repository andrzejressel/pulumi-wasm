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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flow_log {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlowLogArgs {
        /// ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.
        #[builder(into, default)]
        pub deliver_cross_account_role: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Describes the destination options for a flow log. More details below.
        #[builder(into, default)]
        pub destination_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::FlowLogDestinationOptions>,
        >,
        /// Elastic Network Interface ID to attach to
        #[builder(into, default)]
        pub eni_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN for the IAM role that's used to post flow logs to a CloudWatch Logs log group
        #[builder(into, default)]
        pub iam_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the logging destination. Either `log_destination` or `log_group_name` must be set.
        #[builder(into, default)]
        pub log_destination: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the logging destination. Valid values: `cloud-watch-logs`, `s3`, `kinesis-data-firehose`. Default: `cloud-watch-logs`.
        #[builder(into, default)]
        pub log_destination_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The fields to include in the flow log record. Accepted format example: `"$${interface-id} $${srcaddr} $${dstaddr} $${srcport} $${dstport}"`.
        #[builder(into, default)]
        pub log_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// **Deprecated:** Use `log_destination` instead. The name of the CloudWatch log group. Either `log_group_name` or `log_destination` must be set.
        #[builder(into, default)]
        pub log_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum interval of time
        /// during which a flow of packets is captured and aggregated into a flow
        /// log record. Valid Values: `60` seconds (1 minute) or `600` seconds (10
        /// minutes). Default: `600`. When `transit_gateway_id` or `transit_gateway_attachment_id` is specified, `max_aggregation_interval` *must* be 60 seconds (1 minute).
        #[builder(into, default)]
        pub max_aggregation_interval: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Subnet ID to attach to
        #[builder(into, default)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of traffic to capture. Valid values: `ACCEPT`,`REJECT`, `ALL`.
        #[builder(into, default)]
        pub traffic_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Transit Gateway Attachment ID to attach to
        #[builder(into, default)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Transit Gateway ID to attach to
        #[builder(into, default)]
        pub transit_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// VPC ID to attach to
        #[builder(into, default)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FlowLogResult {
        /// The ARN of the Flow Log.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.
        pub deliver_cross_account_role: pulumi_gestalt_rust::Output<Option<String>>,
        /// Describes the destination options for a flow log. More details below.
        pub destination_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::FlowLogDestinationOptions>,
        >,
        /// Elastic Network Interface ID to attach to
        pub eni_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN for the IAM role that's used to post flow logs to a CloudWatch Logs log group
        pub iam_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the logging destination. Either `log_destination` or `log_group_name` must be set.
        pub log_destination: pulumi_gestalt_rust::Output<String>,
        /// The type of the logging destination. Valid values: `cloud-watch-logs`, `s3`, `kinesis-data-firehose`. Default: `cloud-watch-logs`.
        pub log_destination_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The fields to include in the flow log record. Accepted format example: `"$${interface-id} $${srcaddr} $${dstaddr} $${srcport} $${dstport}"`.
        pub log_format: pulumi_gestalt_rust::Output<String>,
        /// **Deprecated:** Use `log_destination` instead. The name of the CloudWatch log group. Either `log_group_name` or `log_destination` must be set.
        pub log_group_name: pulumi_gestalt_rust::Output<String>,
        /// The maximum interval of time
        /// during which a flow of packets is captured and aggregated into a flow
        /// log record. Valid Values: `60` seconds (1 minute) or `600` seconds (10
        /// minutes). Default: `600`. When `transit_gateway_id` or `transit_gateway_attachment_id` is specified, `max_aggregation_interval` *must* be 60 seconds (1 minute).
        pub max_aggregation_interval: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Subnet ID to attach to
        pub subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of traffic to capture. Valid values: `ACCEPT`,`REJECT`, `ALL`.
        pub traffic_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Transit Gateway Attachment ID to attach to
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Transit Gateway ID to attach to
        pub transit_gateway_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// VPC ID to attach to
        pub vpc_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlowLogArgs,
    ) -> FlowLogResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deliver_cross_account_role_binding = args
            .deliver_cross_account_role
            .get_output(context);
        let destination_options_binding = args.destination_options.get_output(context);
        let eni_id_binding = args.eni_id.get_output(context);
        let iam_role_arn_binding = args.iam_role_arn.get_output(context);
        let log_destination_binding = args.log_destination.get_output(context);
        let log_destination_type_binding = args.log_destination_type.get_output(context);
        let log_format_binding = args.log_format.get_output(context);
        let log_group_name_binding = args.log_group_name.get_output(context);
        let max_aggregation_interval_binding = args
            .max_aggregation_interval
            .get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let traffic_type_binding = args.traffic_type.get_output(context);
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(context);
        let transit_gateway_id_binding = args.transit_gateway_id.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/flowLog:FlowLog".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliverCrossAccountRole".into(),
                    value: &deliver_cross_account_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationOptions".into(),
                    value: &destination_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eniId".into(),
                    value: &eni_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logDestination".into(),
                    value: &log_destination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logDestinationType".into(),
                    value: &log_destination_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logFormat".into(),
                    value: &log_format_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroupName".into(),
                    value: &log_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxAggregationInterval".into(),
                    value: &max_aggregation_interval_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficType".into(),
                    value: &traffic_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FlowLogResult {
            arn: o.get_field("arn"),
            deliver_cross_account_role: o.get_field("deliverCrossAccountRole"),
            destination_options: o.get_field("destinationOptions"),
            eni_id: o.get_field("eniId"),
            iam_role_arn: o.get_field("iamRoleArn"),
            log_destination: o.get_field("logDestination"),
            log_destination_type: o.get_field("logDestinationType"),
            log_format: o.get_field("logFormat"),
            log_group_name: o.get_field("logGroupName"),
            max_aggregation_interval: o.get_field("maxAggregationInterval"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            traffic_type: o.get_field("trafficType"),
            transit_gateway_attachment_id: o.get_field("transitGatewayAttachmentId"),
            transit_gateway_id: o.get_field("transitGatewayId"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}

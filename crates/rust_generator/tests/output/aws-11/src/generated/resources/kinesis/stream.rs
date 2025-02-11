/// Provides a Kinesis Stream resource. Amazon Kinesis is a managed service that
/// scales elastically for real-time processing of streaming big data.
///
/// For more details, see the [Amazon Kinesis Documentation](https://aws.amazon.com/documentation/kinesis/).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testStream:
///     type: aws:kinesis:Stream
///     name: test_stream
///     properties:
///       name: kinesis-test
///       shardCount: 1
///       retentionPeriod: 48
///       shardLevelMetrics:
///         - IncomingBytes
///         - OutgoingBytes
///       streamModeDetails:
///         streamMode: PROVISIONED
///       tags:
///         Environment: test
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Kinesis Streams using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:kinesis/stream:Stream test_stream pulumi-kinesis-test
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stream {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StreamArgs {
        /// The Amazon Resource Name (ARN) specifying the Stream (same as `id`)
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The encryption type to use. The only acceptable values are `NONE` or `KMS`. The default value is `NONE`.
        #[builder(into, default)]
        pub encryption_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A boolean that indicates all registered consumers should be deregistered from the stream so that the stream can be destroyed without error. The default value is `false`.
        #[builder(into, default)]
        pub enforce_consumer_deletion: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The GUID for the customer-managed KMS key to use for encryption. You can also use a Kinesis-owned master key by specifying the alias `alias/aws/kinesis`.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A name to identify the stream. This is unique to the AWS account and region the Stream is created in.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Length of time data records are accessible after they are added to the stream. The maximum value of a stream's retention period is 8760 hours. Minimum value is 24. Default is 24.
        #[builder(into, default)]
        pub retention_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The number of shards that the stream will use. If the `stream_mode` is `PROVISIONED`, this field is required.
        /// Amazon has guidelines for specifying the Stream size that should be referenced when creating a Kinesis stream. See [Amazon Kinesis Streams](https://docs.aws.amazon.com/kinesis/latest/dev/amazon-kinesis-streams.html) for more.
        #[builder(into, default)]
        pub shard_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A list of shard-level CloudWatch metrics which can be enabled for the stream. See [Monitoring with CloudWatch](https://docs.aws.amazon.com/streams/latest/dev/monitoring-with-cloudwatch.html) for more. Note that the value ALL should not be used; instead you should provide an explicit list of metrics you wish to enable.
        #[builder(into, default)]
        pub shard_level_metrics: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Indicates the [capacity mode](https://docs.aws.amazon.com/streams/latest/dev/how-do-i-size-a-stream.html) of the data stream. Detailed below.
        #[builder(into, default)]
        pub stream_mode_details: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kinesis::StreamStreamModeDetails>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StreamResult {
        /// The Amazon Resource Name (ARN) specifying the Stream (same as `id`)
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The encryption type to use. The only acceptable values are `NONE` or `KMS`. The default value is `NONE`.
        pub encryption_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A boolean that indicates all registered consumers should be deregistered from the stream so that the stream can be destroyed without error. The default value is `false`.
        pub enforce_consumer_deletion: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The GUID for the customer-managed KMS key to use for encryption. You can also use a Kinesis-owned master key by specifying the alias `alias/aws/kinesis`.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A name to identify the stream. This is unique to the AWS account and region the Stream is created in.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Length of time data records are accessible after they are added to the stream. The maximum value of a stream's retention period is 8760 hours. Minimum value is 24. Default is 24.
        pub retention_period: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The number of shards that the stream will use. If the `stream_mode` is `PROVISIONED`, this field is required.
        /// Amazon has guidelines for specifying the Stream size that should be referenced when creating a Kinesis stream. See [Amazon Kinesis Streams](https://docs.aws.amazon.com/kinesis/latest/dev/amazon-kinesis-streams.html) for more.
        pub shard_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A list of shard-level CloudWatch metrics which can be enabled for the stream. See [Monitoring with CloudWatch](https://docs.aws.amazon.com/streams/latest/dev/monitoring-with-cloudwatch.html) for more. Note that the value ALL should not be used; instead you should provide an explicit list of metrics you wish to enable.
        pub shard_level_metrics: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Indicates the [capacity mode](https://docs.aws.amazon.com/streams/latest/dev/how-do-i-size-a-stream.html) of the data stream. Detailed below.
        pub stream_mode_details: pulumi_gestalt_rust::Output<
            super::super::types::kinesis::StreamStreamModeDetails,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StreamArgs,
    ) -> StreamResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let encryption_type_binding = args.encryption_type.get_output(context);
        let enforce_consumer_deletion_binding = args
            .enforce_consumer_deletion
            .get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let retention_period_binding = args.retention_period.get_output(context);
        let shard_count_binding = args.shard_count.get_output(context);
        let shard_level_metrics_binding = args.shard_level_metrics.get_output(context);
        let stream_mode_details_binding = args.stream_mode_details.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kinesis/stream:Stream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionType".into(),
                    value: &encryption_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enforceConsumerDeletion".into(),
                    value: &enforce_consumer_deletion_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionPeriod".into(),
                    value: &retention_period_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shardCount".into(),
                    value: &shard_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shardLevelMetrics".into(),
                    value: &shard_level_metrics_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamModeDetails".into(),
                    value: &stream_mode_details_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        StreamResult {
            arn: o.get_field("arn"),
            encryption_type: o.get_field("encryptionType"),
            enforce_consumer_deletion: o.get_field("enforceConsumerDeletion"),
            kms_key_id: o.get_field("kmsKeyId"),
            name: o.get_field("name"),
            retention_period: o.get_field("retentionPeriod"),
            shard_count: o.get_field("shardCount"),
            shard_level_metrics: o.get_field("shardLevelMetrics"),
            stream_mode_details: o.get_field("streamModeDetails"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}

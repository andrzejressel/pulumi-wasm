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
pub mod stream {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StreamArgs {
        /// The Amazon Resource Name (ARN) specifying the Stream (same as `id`)
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The encryption type to use. The only acceptable values are `NONE` or `KMS`. The default value is `NONE`.
        #[builder(into, default)]
        pub encryption_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A boolean that indicates all registered consumers should be deregistered from the stream so that the stream can be destroyed without error. The default value is `false`.
        #[builder(into, default)]
        pub enforce_consumer_deletion: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The GUID for the customer-managed KMS key to use for encryption. You can also use a Kinesis-owned master key by specifying the alias `alias/aws/kinesis`.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A name to identify the stream. This is unique to the AWS account and region the Stream is created in.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Length of time data records are accessible after they are added to the stream. The maximum value of a stream's retention period is 8760 hours. Minimum value is 24. Default is 24.
        #[builder(into, default)]
        pub retention_period: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The number of shards that the stream will use. If the `stream_mode` is `PROVISIONED`, this field is required.
        /// Amazon has guidelines for specifying the Stream size that should be referenced when creating a Kinesis stream. See [Amazon Kinesis Streams](https://docs.aws.amazon.com/kinesis/latest/dev/amazon-kinesis-streams.html) for more.
        #[builder(into, default)]
        pub shard_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// A list of shard-level CloudWatch metrics which can be enabled for the stream. See [Monitoring with CloudWatch](https://docs.aws.amazon.com/streams/latest/dev/monitoring-with-cloudwatch.html) for more. Note that the value ALL should not be used; instead you should provide an explicit list of metrics you wish to enable.
        #[builder(into, default)]
        pub shard_level_metrics: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Indicates the [capacity mode](https://docs.aws.amazon.com/streams/latest/dev/how-do-i-size-a-stream.html) of the data stream. Detailed below.
        #[builder(into, default)]
        pub stream_mode_details: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::kinesis::StreamStreamModeDetails>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StreamResult {
        /// The Amazon Resource Name (ARN) specifying the Stream (same as `id`)
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The encryption type to use. The only acceptable values are `NONE` or `KMS`. The default value is `NONE`.
        pub encryption_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A boolean that indicates all registered consumers should be deregistered from the stream so that the stream can be destroyed without error. The default value is `false`.
        pub enforce_consumer_deletion: pulumi_wasm_rust::Output<Option<bool>>,
        /// The GUID for the customer-managed KMS key to use for encryption. You can also use a Kinesis-owned master key by specifying the alias `alias/aws/kinesis`.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A name to identify the stream. This is unique to the AWS account and region the Stream is created in.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Length of time data records are accessible after they are added to the stream. The maximum value of a stream's retention period is 8760 hours. Minimum value is 24. Default is 24.
        pub retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// The number of shards that the stream will use. If the `stream_mode` is `PROVISIONED`, this field is required.
        /// Amazon has guidelines for specifying the Stream size that should be referenced when creating a Kinesis stream. See [Amazon Kinesis Streams](https://docs.aws.amazon.com/kinesis/latest/dev/amazon-kinesis-streams.html) for more.
        pub shard_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// A list of shard-level CloudWatch metrics which can be enabled for the stream. See [Monitoring with CloudWatch](https://docs.aws.amazon.com/streams/latest/dev/monitoring-with-cloudwatch.html) for more. Note that the value ALL should not be used; instead you should provide an explicit list of metrics you wish to enable.
        pub shard_level_metrics: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Indicates the [capacity mode](https://docs.aws.amazon.com/streams/latest/dev/how-do-i-size-a-stream.html) of the data stream. Detailed below.
        pub stream_mode_details: pulumi_wasm_rust::Output<
            super::super::types::kinesis::StreamStreamModeDetails,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StreamArgs,
    ) -> StreamResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let encryption_type_binding = args
            .encryption_type
            .get_output(context)
            .get_inner();
        let enforce_consumer_deletion_binding = args
            .enforce_consumer_deletion
            .get_output(context)
            .get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let retention_period_binding = args
            .retention_period
            .get_output(context)
            .get_inner();
        let shard_count_binding = args.shard_count.get_output(context).get_inner();
        let shard_level_metrics_binding = args
            .shard_level_metrics
            .get_output(context)
            .get_inner();
        let stream_mode_details_binding = args
            .stream_mode_details
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kinesis/stream:Stream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionType".into(),
                    value: &encryption_type_binding,
                },
                register_interface::ObjectField {
                    name: "enforceConsumerDeletion".into(),
                    value: &enforce_consumer_deletion_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "retentionPeriod".into(),
                    value: &retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "shardCount".into(),
                    value: &shard_count_binding,
                },
                register_interface::ObjectField {
                    name: "shardLevelMetrics".into(),
                    value: &shard_level_metrics_binding,
                },
                register_interface::ObjectField {
                    name: "streamModeDetails".into(),
                    value: &stream_mode_details_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "encryptionType".into(),
                },
                register_interface::ResultField {
                    name: "enforceConsumerDeletion".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "retentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "shardCount".into(),
                },
                register_interface::ResultField {
                    name: "shardLevelMetrics".into(),
                },
                register_interface::ResultField {
                    name: "streamModeDetails".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StreamResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            encryption_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionType").unwrap(),
            ),
            enforce_consumer_deletion: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enforceConsumerDeletion").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionPeriod").unwrap(),
            ),
            shard_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shardCount").unwrap(),
            ),
            shard_level_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shardLevelMetrics").unwrap(),
            ),
            stream_mode_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamModeDetails").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}

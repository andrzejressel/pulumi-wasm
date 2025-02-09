/// Provides an AWS Quantum Ledger Database (QLDB) Stream resource
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:qldb:Stream
///     properties:
///       ledgerName: existing-ledger-name
///       streamName: sample-ledger-stream
///       roleArn: sample-role-arn
///       inclusiveStartTime: 2021-01-01T00:00:00Z
///       kinesisConfiguration:
///         aggregationEnabled: false
///         streamArn: arn:aws:kinesis:us-east-1:xxxxxxxxxxxx:stream/example-kinesis-stream
///       tags:
///         example: tag
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stream {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StreamArgs {
        /// The exclusive date and time that specifies when the stream ends. If you don't define this parameter, the stream runs indefinitely until you cancel it. It must be in ISO 8601 date and time format and in Universal Coordinated Time (UTC). For example: `"2019-06-13T21:36:34Z"`.
        #[builder(into, default)]
        pub exclusive_end_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The inclusive start date and time from which to start streaming journal data. This parameter must be in ISO 8601 date and time format and in Universal Coordinated Time (UTC). For example: `"2019-06-13T21:36:34Z"`.  This cannot be in the future and must be before `exclusive_end_time`.  If you provide a value that is before the ledger's `CreationDateTime`, QLDB effectively defaults it to the ledger's `CreationDateTime`.
        #[builder(into)]
        pub inclusive_start_time: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The configuration settings of the Kinesis Data Streams destination for your stream request. Documented below.
        #[builder(into)]
        pub kinesis_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::qldb::StreamKinesisConfiguration,
        >,
        /// The name of the QLDB ledger.
        #[builder(into)]
        pub ledger_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the IAM role that grants QLDB permissions for a journal stream to write data records to a Kinesis Data Streams resource.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name that you want to assign to the QLDB journal stream. User-defined names can help identify and indicate the purpose of a stream.  Your stream name must be unique among other active streams for a given ledger. Stream names have the same naming constraints as ledger names, as defined in the [Amazon QLDB Developer Guide](https://docs.aws.amazon.com/qldb/latest/developerguide/limits.html#limits.naming).
        #[builder(into)]
        pub stream_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct StreamResult {
        /// The ARN of the QLDB Stream.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The exclusive date and time that specifies when the stream ends. If you don't define this parameter, the stream runs indefinitely until you cancel it. It must be in ISO 8601 date and time format and in Universal Coordinated Time (UTC). For example: `"2019-06-13T21:36:34Z"`.
        pub exclusive_end_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// The inclusive start date and time from which to start streaming journal data. This parameter must be in ISO 8601 date and time format and in Universal Coordinated Time (UTC). For example: `"2019-06-13T21:36:34Z"`.  This cannot be in the future and must be before `exclusive_end_time`.  If you provide a value that is before the ledger's `CreationDateTime`, QLDB effectively defaults it to the ledger's `CreationDateTime`.
        pub inclusive_start_time: pulumi_gestalt_rust::Output<String>,
        /// The configuration settings of the Kinesis Data Streams destination for your stream request. Documented below.
        pub kinesis_configuration: pulumi_gestalt_rust::Output<
            super::super::types::qldb::StreamKinesisConfiguration,
        >,
        /// The name of the QLDB ledger.
        pub ledger_name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IAM role that grants QLDB permissions for a journal stream to write data records to a Kinesis Data Streams resource.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// The name that you want to assign to the QLDB journal stream. User-defined names can help identify and indicate the purpose of a stream.  Your stream name must be unique among other active streams for a given ledger. Stream names have the same naming constraints as ledger names, as defined in the [Amazon QLDB Developer Guide](https://docs.aws.amazon.com/qldb/latest/developerguide/limits.html#limits.naming).
        pub stream_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: StreamArgs,
    ) -> StreamResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let exclusive_end_time_binding_1 = args.exclusive_end_time.get_output(context);
        let exclusive_end_time_binding = exclusive_end_time_binding_1.get_inner();
        let inclusive_start_time_binding_1 = args
            .inclusive_start_time
            .get_output(context);
        let inclusive_start_time_binding = inclusive_start_time_binding_1.get_inner();
        let kinesis_configuration_binding_1 = args
            .kinesis_configuration
            .get_output(context);
        let kinesis_configuration_binding = kinesis_configuration_binding_1.get_inner();
        let ledger_name_binding_1 = args.ledger_name.get_output(context);
        let ledger_name_binding = ledger_name_binding_1.get_inner();
        let role_arn_binding_1 = args.role_arn.get_output(context);
        let role_arn_binding = role_arn_binding_1.get_inner();
        let stream_name_binding_1 = args.stream_name.get_output(context);
        let stream_name_binding = stream_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:qldb/stream:Stream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "exclusiveEndTime".into(),
                    value: &exclusive_end_time_binding,
                },
                register_interface::ObjectField {
                    name: "inclusiveStartTime".into(),
                    value: &inclusive_start_time_binding,
                },
                register_interface::ObjectField {
                    name: "kinesisConfiguration".into(),
                    value: &kinesis_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "ledgerName".into(),
                    value: &ledger_name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "streamName".into(),
                    value: &stream_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StreamResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            exclusive_end_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("exclusiveEndTime"),
            ),
            inclusive_start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inclusiveStartTime"),
            ),
            kinesis_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kinesisConfiguration"),
            ),
            ledger_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ledgerName"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            stream_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}

/// Provides a resource to manage a Kinesis Stream Consumer.
///
/// > **Note:** You can register up to 20 consumers per stream. A given consumer can only be registered with one stream at a time.
///
/// For more details, see the [Amazon Kinesis Stream Consumer Documentation](https://docs.aws.amazon.com/streams/latest/dev/amazon-kinesis-consumers.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = stream::create(
///         "example",
///         StreamArgs::builder().name("example-stream").shard_count(1).build_struct(),
///     );
///     let exampleStreamConsumer = stream_consumer::create(
///         "exampleStreamConsumer",
///         StreamConsumerArgs::builder()
///             .name("example-consumer")
///             .stream_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Kinesis Stream Consumers using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:kinesis/streamConsumer:StreamConsumer example arn:aws:kinesis:us-west-2:123456789012:stream/example/consumer/example:1616044553
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stream_consumer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StreamConsumerArgs {
        /// Name of the stream consumer.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the data stream the consumer is registered with.
        #[builder(into)]
        pub stream_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StreamConsumerResult {
        /// Amazon Resource Name (ARN) of the stream consumer.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Approximate timestamp in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) of when the stream consumer was created.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Name of the stream consumer.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the data stream the consumer is registered with.
        pub stream_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StreamConsumerArgs,
    ) -> StreamConsumerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let stream_arn_binding = args.stream_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kinesis/streamConsumer:StreamConsumer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamArn".into(),
                    value: stream_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        StreamConsumerResult {
            arn: o.get_field("arn"),
            creation_timestamp: o.get_field("creationTimestamp"),
            name: o.get_field("name"),
            stream_arn: o.get_field("streamArn"),
        }
    }
}

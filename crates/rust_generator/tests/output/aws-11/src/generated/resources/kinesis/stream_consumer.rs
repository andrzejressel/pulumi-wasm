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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: StreamConsumerArgs,
    ) -> StreamConsumerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let stream_arn_binding = args.stream_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kinesis/streamConsumer:StreamConsumer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "streamArn".into(),
                    value: &stream_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StreamConsumerResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            stream_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamArn"),
            ),
        }
    }
}

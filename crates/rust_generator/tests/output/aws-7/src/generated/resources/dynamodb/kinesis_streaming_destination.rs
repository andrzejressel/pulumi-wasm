/// Enables a [Kinesis streaming destination](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/kds.html) for data replication of a DynamoDB table.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = table::create(
///         "example",
///         TableArgs::builder()
///             .attributes(
///                 vec![TableAttribute::builder().name("id"). type ("S").build_struct(),],
///             )
///             .hash_key("id")
///             .name("orders")
///             .build_struct(),
///     );
///     let exampleKinesisStreamingDestination = kinesis_streaming_destination::create(
///         "exampleKinesisStreamingDestination",
///         KinesisStreamingDestinationArgs::builder()
///             .approximate_creation_date_time_precision("MICROSECOND")
///             .stream_arn("${exampleStream.arn}")
///             .table_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleStream = stream::create(
///         "exampleStream",
///         StreamArgs::builder().name("order_item_changes").shard_count(1).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DynamoDB Kinesis Streaming Destinations using the `table_name` and `stream_arn` separated by `,`. For example:
///
/// ```sh
/// $ pulumi import aws:dynamodb/kinesisStreamingDestination:KinesisStreamingDestination example example,arn:aws:kinesis:us-east-1:111122223333:exampleStreamName
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kinesis_streaming_destination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KinesisStreamingDestinationArgs {
        /// Toggle for the precision of Kinesis data stream timestamp. Valid values: `MILLISECOND` and `MICROSECOND`.
        #[builder(into, default)]
        pub approximate_creation_date_time_precision: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ARN for a Kinesis data stream. This must exist in the same account and region as the DynamoDB table.
        #[builder(into)]
        pub stream_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the DynamoDB table. There can only be one Kinesis streaming destination for a given DynamoDB table.
        #[builder(into)]
        pub table_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct KinesisStreamingDestinationResult {
        /// Toggle for the precision of Kinesis data stream timestamp. Valid values: `MILLISECOND` and `MICROSECOND`.
        pub approximate_creation_date_time_precision: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The ARN for a Kinesis data stream. This must exist in the same account and region as the DynamoDB table.
        pub stream_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the DynamoDB table. There can only be one Kinesis streaming destination for a given DynamoDB table.
        pub table_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KinesisStreamingDestinationArgs,
    ) -> KinesisStreamingDestinationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let approximate_creation_date_time_precision_binding = args
            .approximate_creation_date_time_precision
            .get_output(context);
        let stream_arn_binding = args.stream_arn.get_output(context);
        let table_name_binding = args.table_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dynamodb/kinesisStreamingDestination:KinesisStreamingDestination"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "approximateCreationDateTimePrecision".into(),
                    value: approximate_creation_date_time_precision_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamArn".into(),
                    value: stream_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: table_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        KinesisStreamingDestinationResult {
            approximate_creation_date_time_precision: o
                .get_field("approximateCreationDateTimePrecision"),
            stream_arn: o.get_field("streamArn"),
            table_name: o.get_field("tableName"),
        }
    }
}

/// Enables a [Kinesis streaming destination](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/kds.html) for data replication of a DynamoDB table.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod kinesis_streaming_destination {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KinesisStreamingDestinationArgs {
        /// Toggle for the precision of Kinesis data stream timestamp. Valid values: `MILLISECOND` and `MICROSECOND`.
        #[builder(into, default)]
        pub approximate_creation_date_time_precision: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The ARN for a Kinesis data stream. This must exist in the same account and region as the DynamoDB table.
        #[builder(into)]
        pub stream_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the DynamoDB table. There can only be one Kinesis streaming destination for a given DynamoDB table.
        #[builder(into)]
        pub table_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct KinesisStreamingDestinationResult {
        /// Toggle for the precision of Kinesis data stream timestamp. Valid values: `MILLISECOND` and `MICROSECOND`.
        pub approximate_creation_date_time_precision: pulumi_wasm_rust::Output<String>,
        /// The ARN for a Kinesis data stream. This must exist in the same account and region as the DynamoDB table.
        pub stream_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the DynamoDB table. There can only be one Kinesis streaming destination for a given DynamoDB table.
        pub table_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: KinesisStreamingDestinationArgs,
    ) -> KinesisStreamingDestinationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let approximate_creation_date_time_precision_binding = args
            .approximate_creation_date_time_precision
            .get_inner();
        let stream_arn_binding = args.stream_arn.get_inner();
        let table_name_binding = args.table_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dynamodb/kinesisStreamingDestination:KinesisStreamingDestination"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "approximateCreationDateTimePrecision".into(),
                    value: &approximate_creation_date_time_precision_binding,
                },
                register_interface::ObjectField {
                    name: "streamArn".into(),
                    value: &stream_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "approximateCreationDateTimePrecision".into(),
                },
                register_interface::ResultField {
                    name: "streamArn".into(),
                },
                register_interface::ResultField {
                    name: "tableName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KinesisStreamingDestinationResult {
            approximate_creation_date_time_precision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approximateCreationDateTimePrecision").unwrap(),
            ),
            stream_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamArn").unwrap(),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableName").unwrap(),
            ),
        }
    }
}
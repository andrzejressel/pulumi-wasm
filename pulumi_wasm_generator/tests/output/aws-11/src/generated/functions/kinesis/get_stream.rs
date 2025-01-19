pub mod get_stream {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStreamArgs {
        /// Name of the Kinesis Stream.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assigned to the stream.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetStreamResult {
        /// ARN of the Kinesis Stream (same as id).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// List of shard ids in the CLOSED state. See [Shard State](https://docs.aws.amazon.com/streams/latest/dev/kinesis-using-sdk-java-after-resharding.html#kinesis-using-sdk-java-resharding-data-routing) for more.
        pub closed_shards: pulumi_wasm_rust::Output<Vec<String>>,
        /// Approximate UNIX timestamp that the stream was created.
        pub creation_timestamp: pulumi_wasm_rust::Output<i32>,
        /// Encryption type used.
        pub encryption_type: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// GUID for the customer-managed AWS KMS key to use for encryption.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Kinesis Stream.
        pub name: pulumi_wasm_rust::Output<String>,
        /// List of shard ids in the OPEN state. See [Shard State](https://docs.aws.amazon.com/streams/latest/dev/kinesis-using-sdk-java-after-resharding.html#kinesis-using-sdk-java-resharding-data-routing) for more.
        pub open_shards: pulumi_wasm_rust::Output<Vec<String>>,
        /// Length of time (in hours) data records are accessible after they are added to the stream.
        pub retention_period: pulumi_wasm_rust::Output<i32>,
        /// List of shard-level CloudWatch metrics which are enabled for the stream. See [Monitoring with CloudWatch](https://docs.aws.amazon.com/streams/latest/dev/monitoring-with-cloudwatch.html) for more.
        pub shard_level_metrics: pulumi_wasm_rust::Output<Vec<String>>,
        /// Current status of the stream. The stream status is one of CREATING, DELETING, ACTIVE, or UPDATING.
        pub status: pulumi_wasm_rust::Output<String>,
        /// [Capacity mode](https://docs.aws.amazon.com/streams/latest/dev/how-do-i-size-a-stream.html) of the data stream. Detailed below.
        pub stream_mode_details: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::kinesis::GetStreamStreamModeDetail>,
        >,
        /// Map of tags to assigned to the stream.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetStreamArgs) -> GetStreamResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kinesis/getStream:getStream".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "closedShards".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "encryptionType".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "openShards".into(),
                },
                register_interface::ResultField {
                    name: "retentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "shardLevelMetrics".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "streamModeDetails".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetStreamResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            closed_shards: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("closedShards").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            encryption_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionType").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            open_shards: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("openShards").unwrap(),
            ),
            retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionPeriod").unwrap(),
            ),
            shard_level_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shardLevelMetrics").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            stream_mode_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamModeDetails").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}

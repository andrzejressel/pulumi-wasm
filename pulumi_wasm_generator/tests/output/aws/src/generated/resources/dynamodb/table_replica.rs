/// Provides a DynamoDB table replica resource for [DynamoDB Global Tables V2 (version 2019.11.21)](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html).
///
/// > **Note:** Use `lifecycle` `ignore_changes` for `replica` in the associated aws.dynamodb.Table configuration.
///
/// > **Note:** Do not use the `replica` configuration block of aws.dynamodb.Table together with this resource as the two configuration options are mutually exclusive.
///
/// ## Example Usage
///
/// ### Basic Example
///
/// ```yaml
/// resources:
///   example:
///     type: aws:dynamodb:Table
///     properties:
///       name: TestTable
///       hashKey: BrodoBaggins
///       billingMode: PAY_PER_REQUEST
///       streamEnabled: true
///       streamViewType: NEW_AND_OLD_IMAGES
///       attributes:
///         - name: BrodoBaggins
///           type: S
///   exampleTableReplica:
///     type: aws:dynamodb:TableReplica
///     name: example
///     properties:
///       globalTableArn: ${example.arn}
///       tags:
///         Name: IZPAWS
///         Pozo: Amargo
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DynamoDB table replicas using the `table-name:main-region`. For example:
///
/// ~> __Note:__ When importing, use the region where the initial or _main_ global table resides, _not_ the region of the replica.
///
/// ```sh
/// $ pulumi import aws:dynamodb/tableReplica:TableReplica example TestTable:us-west-2
/// ```
pub mod table_replica {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableReplicaArgs {
        /// ARN of the _main_ or global table which this resource will replicate.
        ///
        /// Optional arguments:
        #[builder(into)]
        pub global_table_arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the CMK that should be used for the AWS KMS encryption. This argument should only be used if the key is different from the default KMS-managed DynamoDB key, `alias/aws/dynamodb`. **Note:** This attribute will _not_ be populated with the ARN of _default_ keys.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to enable Point In Time Recovery for the replica. Default is `false`.
        #[builder(into, default)]
        pub point_in_time_recovery: pulumi_wasm_rust::Output<Option<bool>>,
        /// Storage class of the table replica. Valid values are `STANDARD` and `STANDARD_INFREQUENT_ACCESS`. If not used, the table replica will use the same class as the global table.
        #[builder(into, default)]
        pub table_class_override: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to populate on the created table. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TableReplicaResult {
        /// ARN of the table replica.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the _main_ or global table which this resource will replicate.
        ///
        /// Optional arguments:
        pub global_table_arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the CMK that should be used for the AWS KMS encryption. This argument should only be used if the key is different from the default KMS-managed DynamoDB key, `alias/aws/dynamodb`. **Note:** This attribute will _not_ be populated with the ARN of _default_ keys.
        pub kms_key_arn: pulumi_wasm_rust::Output<String>,
        /// Whether to enable Point In Time Recovery for the replica. Default is `false`.
        pub point_in_time_recovery: pulumi_wasm_rust::Output<Option<bool>>,
        /// Storage class of the table replica. Valid values are `STANDARD` and `STANDARD_INFREQUENT_ACCESS`. If not used, the table replica will use the same class as the global table.
        pub table_class_override: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to populate on the created table. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TableReplicaArgs) -> TableReplicaResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let global_table_arn_binding = args.global_table_arn.get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_inner();
        let point_in_time_recovery_binding = args.point_in_time_recovery.get_inner();
        let table_class_override_binding = args.table_class_override.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dynamodb/tableReplica:TableReplica".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "globalTableArn".into(),
                    value: &global_table_arn_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "pointInTimeRecovery".into(),
                    value: &point_in_time_recovery_binding,
                },
                register_interface::ObjectField {
                    name: "tableClassOverride".into(),
                    value: &table_class_override_binding,
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
                    name: "globalTableArn".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "pointInTimeRecovery".into(),
                },
                register_interface::ResultField {
                    name: "tableClassOverride".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TableReplicaResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            global_table_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalTableArn").unwrap(),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
            ),
            point_in_time_recovery: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pointInTimeRecovery").unwrap(),
            ),
            table_class_override: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableClassOverride").unwrap(),
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
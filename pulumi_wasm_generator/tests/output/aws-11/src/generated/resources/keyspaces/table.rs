/// Provides a Keyspaces Table.
///
/// More information about Keyspaces tables can be found in the [Keyspaces Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/working-with-tables.html).
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
///             .keyspace_name("${exampleAwsKeyspacesKeyspace.name}")
///             .schema_definition(
///                 TableSchemaDefinition::builder()
///                     .columns(
///                         vec![
///                             TableSchemaDefinitionColumn::builder().name("Message"). type
///                             ("ASCII").build_struct(),
///                         ],
///                     )
///                     .partitionKeys(
///                         vec![
///                             TableSchemaDefinitionPartitionKey::builder().name("Message")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .table_name("my_table")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a table using the `keyspace_name` and `table_name` separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:keyspaces/table:Table example my_keyspace/my_table
/// ```
pub mod table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableArgs {
        /// Specifies the read/write throughput capacity mode for the table.
        #[builder(into, default)]
        pub capacity_specification: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TableCapacitySpecification>,
        >,
        /// Enables client-side timestamps for the table. By default, the setting is disabled.
        #[builder(into, default)]
        pub client_side_timestamps: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TableClientSideTimestamps>,
        >,
        /// A description of the table.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TableComment>,
        >,
        /// The default Time to Live setting in seconds for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL-how-it-works.html#ttl-howitworks_default_ttl).
        #[builder(into, default)]
        pub default_time_to_live: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specifies how the encryption key for encryption at rest is managed for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html).
        #[builder(into, default)]
        pub encryption_specification: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TableEncryptionSpecification>,
        >,
        /// The name of the keyspace that the table is going to be created in.
        #[builder(into)]
        pub keyspace_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies if point-in-time recovery is enabled or disabled for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html).
        #[builder(into, default)]
        pub point_in_time_recovery: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TablePointInTimeRecovery>,
        >,
        /// Describes the schema of the table.
        #[builder(into)]
        pub schema_definition: pulumi_wasm_rust::InputOrOutput<
            super::super::types::keyspaces::TableSchemaDefinition,
        >,
        /// The name of the table.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub table_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enables Time to Live custom settings for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL.html).
        #[builder(into, default)]
        pub ttl: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::keyspaces::TableTtl>,
        >,
    }
    #[allow(dead_code)]
    pub struct TableResult {
        /// The ARN of the table.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies the read/write throughput capacity mode for the table.
        pub capacity_specification: pulumi_wasm_rust::Output<
            super::super::types::keyspaces::TableCapacitySpecification,
        >,
        /// Enables client-side timestamps for the table. By default, the setting is disabled.
        pub client_side_timestamps: pulumi_wasm_rust::Output<
            Option<super::super::types::keyspaces::TableClientSideTimestamps>,
        >,
        /// A description of the table.
        pub comment: pulumi_wasm_rust::Output<
            super::super::types::keyspaces::TableComment,
        >,
        /// The default Time to Live setting in seconds for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL-how-it-works.html#ttl-howitworks_default_ttl).
        pub default_time_to_live: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies how the encryption key for encryption at rest is managed for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html).
        pub encryption_specification: pulumi_wasm_rust::Output<
            super::super::types::keyspaces::TableEncryptionSpecification,
        >,
        /// The name of the keyspace that the table is going to be created in.
        pub keyspace_name: pulumi_wasm_rust::Output<String>,
        /// Specifies if point-in-time recovery is enabled or disabled for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html).
        pub point_in_time_recovery: pulumi_wasm_rust::Output<
            super::super::types::keyspaces::TablePointInTimeRecovery,
        >,
        /// Describes the schema of the table.
        pub schema_definition: pulumi_wasm_rust::Output<
            super::super::types::keyspaces::TableSchemaDefinition,
        >,
        /// The name of the table.
        ///
        /// The following arguments are optional:
        pub table_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enables Time to Live custom settings for the table. More information can be found in the [Developer Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL.html).
        pub ttl: pulumi_wasm_rust::Output<
            Option<super::super::types::keyspaces::TableTtl>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TableArgs,
    ) -> TableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_specification_binding = args
            .capacity_specification
            .get_output(context)
            .get_inner();
        let client_side_timestamps_binding = args
            .client_side_timestamps
            .get_output(context)
            .get_inner();
        let comment_binding = args.comment.get_output(context).get_inner();
        let default_time_to_live_binding = args
            .default_time_to_live
            .get_output(context)
            .get_inner();
        let encryption_specification_binding = args
            .encryption_specification
            .get_output(context)
            .get_inner();
        let keyspace_name_binding = args.keyspace_name.get_output(context).get_inner();
        let point_in_time_recovery_binding = args
            .point_in_time_recovery
            .get_output(context)
            .get_inner();
        let schema_definition_binding = args
            .schema_definition
            .get_output(context)
            .get_inner();
        let table_name_binding = args.table_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let ttl_binding = args.ttl.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:keyspaces/table:Table".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacitySpecification".into(),
                    value: &capacity_specification_binding,
                },
                register_interface::ObjectField {
                    name: "clientSideTimestamps".into(),
                    value: &client_side_timestamps_binding,
                },
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "defaultTimeToLive".into(),
                    value: &default_time_to_live_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionSpecification".into(),
                    value: &encryption_specification_binding,
                },
                register_interface::ObjectField {
                    name: "keyspaceName".into(),
                    value: &keyspace_name_binding,
                },
                register_interface::ObjectField {
                    name: "pointInTimeRecovery".into(),
                    value: &point_in_time_recovery_binding,
                },
                register_interface::ObjectField {
                    name: "schemaDefinition".into(),
                    value: &schema_definition_binding,
                },
                register_interface::ObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TableResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            capacity_specification: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capacitySpecification"),
            ),
            client_side_timestamps: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientSideTimestamps"),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            default_time_to_live: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultTimeToLive"),
            ),
            encryption_specification: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionSpecification"),
            ),
            keyspace_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyspaceName"),
            ),
            point_in_time_recovery: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pointInTimeRecovery"),
            ),
            schema_definition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("schemaDefinition"),
            ),
            table_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tableName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(o.extract_field("ttl")),
        }
    }
}

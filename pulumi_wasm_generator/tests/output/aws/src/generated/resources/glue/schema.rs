/// Provides a Glue Schema resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = schema::create(
///         "example",
///         SchemaArgs::builder()
///             .compatibility("NONE")
///             .data_format("AVRO")
///             .registry_arn("${test.arn}")
///             .schema_definition(
///                 "{\"type\": \"record\", \"name\": \"r1\", \"fields\": [ {\"name\": \"f1\", \"type\": \"int\"}, {\"name\": \"f2\", \"type\": \"string\"} ]}",
///             )
///             .schema_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Registries using `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/schema:Schema example arn:aws:glue:us-west-2:123456789012:schema/example/example
/// ```
pub mod schema {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchemaArgs {
        /// The compatibility mode of the schema. Values values are: `NONE`, `DISABLED`, `BACKWARD`, `BACKWARD_ALL`, `FORWARD`, `FORWARD_ALL`, `FULL`, and `FULL_ALL`.
        #[builder(into)]
        pub compatibility: pulumi_wasm_rust::Output<String>,
        /// The data format of the schema definition. Valid values are `AVRO`, `JSON` and `PROTOBUF`.
        #[builder(into)]
        pub data_format: pulumi_wasm_rust::Output<String>,
        /// A description of the schema.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the Glue Registry to create the schema in.
        #[builder(into, default)]
        pub registry_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The schema definition using the `data_format` setting for `schema_name`.
        #[builder(into)]
        pub schema_definition: pulumi_wasm_rust::Output<String>,
        /// The Name of the schema.
        #[builder(into)]
        pub schema_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SchemaResult {
        /// Amazon Resource Name (ARN) of the schema.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The compatibility mode of the schema. Values values are: `NONE`, `DISABLED`, `BACKWARD`, `BACKWARD_ALL`, `FORWARD`, `FORWARD_ALL`, `FULL`, and `FULL_ALL`.
        pub compatibility: pulumi_wasm_rust::Output<String>,
        /// The data format of the schema definition. Valid values are `AVRO`, `JSON` and `PROTOBUF`.
        pub data_format: pulumi_wasm_rust::Output<String>,
        /// A description of the schema.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The latest version of the schema associated with the returned schema definition.
        pub latest_schema_version: pulumi_wasm_rust::Output<i32>,
        /// The next version of the schema associated with the returned schema definition.
        pub next_schema_version: pulumi_wasm_rust::Output<i32>,
        /// The ARN of the Glue Registry to create the schema in.
        pub registry_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the Glue Registry.
        pub registry_name: pulumi_wasm_rust::Output<String>,
        /// The version number of the checkpoint (the last time the compatibility mode was changed).
        pub schema_checkpoint: pulumi_wasm_rust::Output<i32>,
        /// The schema definition using the `data_format` setting for `schema_name`.
        pub schema_definition: pulumi_wasm_rust::Output<String>,
        /// The Name of the schema.
        pub schema_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: SchemaArgs) -> SchemaResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let compatibility_binding = args.compatibility.get_inner();
        let data_format_binding = args.data_format.get_inner();
        let description_binding = args.description.get_inner();
        let registry_arn_binding = args.registry_arn.get_inner();
        let schema_definition_binding = args.schema_definition.get_inner();
        let schema_name_binding = args.schema_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/schema:Schema".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "compatibility".into(),
                    value: &compatibility_binding,
                },
                register_interface::ObjectField {
                    name: "dataFormat".into(),
                    value: &data_format_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "registryArn".into(),
                    value: &registry_arn_binding,
                },
                register_interface::ObjectField {
                    name: "schemaDefinition".into(),
                    value: &schema_definition_binding,
                },
                register_interface::ObjectField {
                    name: "schemaName".into(),
                    value: &schema_name_binding,
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
                    name: "compatibility".into(),
                },
                register_interface::ResultField {
                    name: "dataFormat".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "latestSchemaVersion".into(),
                },
                register_interface::ResultField {
                    name: "nextSchemaVersion".into(),
                },
                register_interface::ResultField {
                    name: "registryArn".into(),
                },
                register_interface::ResultField {
                    name: "registryName".into(),
                },
                register_interface::ResultField {
                    name: "schemaCheckpoint".into(),
                },
                register_interface::ResultField {
                    name: "schemaDefinition".into(),
                },
                register_interface::ResultField {
                    name: "schemaName".into(),
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
        SchemaResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            compatibility: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compatibility").unwrap(),
            ),
            data_format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFormat").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            latest_schema_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestSchemaVersion").unwrap(),
            ),
            next_schema_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextSchemaVersion").unwrap(),
            ),
            registry_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryArn").unwrap(),
            ),
            registry_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryName").unwrap(),
            ),
            schema_checkpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemaCheckpoint").unwrap(),
            ),
            schema_definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemaDefinition").unwrap(),
            ),
            schema_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemaName").unwrap(),
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
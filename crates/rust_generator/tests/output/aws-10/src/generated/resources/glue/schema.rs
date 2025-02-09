/// Provides a Glue Schema resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod schema {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchemaArgs {
        /// The compatibility mode of the schema. Values values are: `NONE`, `DISABLED`, `BACKWARD`, `BACKWARD_ALL`, `FORWARD`, `FORWARD_ALL`, `FULL`, and `FULL_ALL`.
        #[builder(into)]
        pub compatibility: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The data format of the schema definition. Valid values are `AVRO`, `JSON` and `PROTOBUF`.
        #[builder(into)]
        pub data_format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description of the schema.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the Glue Registry to create the schema in.
        #[builder(into, default)]
        pub registry_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The schema definition using the `data_format` setting for `schema_name`.
        #[builder(into)]
        pub schema_definition: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the schema.
        #[builder(into)]
        pub schema_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SchemaResult {
        /// Amazon Resource Name (ARN) of the schema.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The compatibility mode of the schema. Values values are: `NONE`, `DISABLED`, `BACKWARD`, `BACKWARD_ALL`, `FORWARD`, `FORWARD_ALL`, `FULL`, and `FULL_ALL`.
        pub compatibility: pulumi_gestalt_rust::Output<String>,
        /// The data format of the schema definition. Valid values are `AVRO`, `JSON` and `PROTOBUF`.
        pub data_format: pulumi_gestalt_rust::Output<String>,
        /// A description of the schema.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The latest version of the schema associated with the returned schema definition.
        pub latest_schema_version: pulumi_gestalt_rust::Output<i32>,
        /// The next version of the schema associated with the returned schema definition.
        pub next_schema_version: pulumi_gestalt_rust::Output<i32>,
        /// The ARN of the Glue Registry to create the schema in.
        pub registry_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Glue Registry.
        pub registry_name: pulumi_gestalt_rust::Output<String>,
        /// The version number of the checkpoint (the last time the compatibility mode was changed).
        pub schema_checkpoint: pulumi_gestalt_rust::Output<i32>,
        /// The schema definition using the `data_format` setting for `schema_name`.
        pub schema_definition: pulumi_gestalt_rust::Output<String>,
        /// The Name of the schema.
        pub schema_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: SchemaArgs,
    ) -> SchemaResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let compatibility_binding_1 = args.compatibility.get_output(context);
        let compatibility_binding = compatibility_binding_1.get_inner();
        let data_format_binding_1 = args.data_format.get_output(context);
        let data_format_binding = data_format_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let registry_arn_binding_1 = args.registry_arn.get_output(context);
        let registry_arn_binding = registry_arn_binding_1.get_inner();
        let schema_definition_binding_1 = args.schema_definition.get_output(context);
        let schema_definition_binding = schema_definition_binding_1.get_inner();
        let schema_name_binding_1 = args.schema_name.get_output(context);
        let schema_name_binding = schema_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/schema:Schema".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SchemaResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            compatibility: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("compatibility"),
            ),
            data_format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataFormat"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            latest_schema_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("latestSchemaVersion"),
            ),
            next_schema_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nextSchemaVersion"),
            ),
            registry_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registryArn"),
            ),
            registry_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registryName"),
            ),
            schema_checkpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schemaCheckpoint"),
            ),
            schema_definition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schemaDefinition"),
            ),
            schema_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schemaName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}

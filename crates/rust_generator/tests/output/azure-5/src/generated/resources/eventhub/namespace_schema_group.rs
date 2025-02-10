/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("East US")
///             .name("exampleRG-ehn-schemaGroup")
///             .build_struct(),
///     );
///     let test = event_hub_namespace::create(
///         "test",
///         EventHubNamespaceArgs::builder()
///             .location("${testAzurermResourceGroup.location}")
///             .name("example-ehn-schemaGroup")
///             .resource_group_name("${testAzurermResourceGroup.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let testNamespaceSchemaGroup = namespace_schema_group::create(
///         "testNamespaceSchemaGroup",
///         NamespaceSchemaGroupArgs::builder()
///             .name("example-schemaGroup")
///             .namespace_id("${test.id}")
///             .schema_compatibility("Forward")
///             .schema_type("Avro")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Schema Group for a EventHub Namespace can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/namespaceSchemaGroup:NamespaceSchemaGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/schemaGroups/group1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod namespace_schema_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceSchemaGroupArgs {
        /// Specifies the name of this schema group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the EventHub Namespace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub namespace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the compatibility of this schema group. Possible values are `None`, `Backward`, `Forward`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub schema_compatibility: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Type of this schema group. Possible values are `Avro`, `Unknown`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub schema_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NamespaceSchemaGroupResult {
        /// Specifies the name of this schema group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the EventHub Namespace. Changing this forces a new resource to be created.
        pub namespace_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the compatibility of this schema group. Possible values are `None`, `Backward`, `Forward`. Changing this forces a new resource to be created.
        pub schema_compatibility: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Type of this schema group. Possible values are `Avro`, `Unknown`. Changing this forces a new resource to be created.
        pub schema_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NamespaceSchemaGroupArgs,
    ) -> NamespaceSchemaGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let namespace_id_binding = args.namespace_id.get_output(context);
        let schema_compatibility_binding = args.schema_compatibility.get_output(context);
        let schema_type_binding = args.schema_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventhub/namespaceSchemaGroup:NamespaceSchemaGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceId".into(),
                    value: namespace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schemaCompatibility".into(),
                    value: schema_compatibility_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schemaType".into(),
                    value: schema_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NamespaceSchemaGroupResult {
            name: o.get_field("name"),
            namespace_id: o.get_field("namespaceId"),
            schema_compatibility: o.get_field("schemaCompatibility"),
            schema_type: o.get_field("schemaType"),
        }
    }
}

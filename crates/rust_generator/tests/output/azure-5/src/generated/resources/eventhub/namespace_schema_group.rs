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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NamespaceSchemaGroupArgs,
    ) -> NamespaceSchemaGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_id_binding = args.namespace_id.get_output(context).get_inner();
        let schema_compatibility_binding = args
            .schema_compatibility
            .get_output(context)
            .get_inner();
        let schema_type_binding = args.schema_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventhub/namespaceSchemaGroup:NamespaceSchemaGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceId".into(),
                    value: &namespace_id_binding,
                },
                register_interface::ObjectField {
                    name: "schemaCompatibility".into(),
                    value: &schema_compatibility_binding,
                },
                register_interface::ObjectField {
                    name: "schemaType".into(),
                    value: &schema_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NamespaceSchemaGroupResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            namespace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespaceId"),
            ),
            schema_compatibility: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schemaCompatibility"),
            ),
            schema_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schemaType"),
            ),
        }
    }
}

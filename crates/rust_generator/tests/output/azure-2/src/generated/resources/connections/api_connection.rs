/// Manages an API Connection.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleNamespace:
///     type: azure:servicebus:Namespace
///     name: example
///     properties:
///       name: acctestsbn-conn-example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku: Basic
///   exampleApiConnection:
///     type: azure:connections:ApiConnection
///     name: example
///     properties:
///       name: example-connection
///       resourceGroupName: ${exampleResourceGroup.name}
///       managedApiId: ${example.id}
///       displayName: Example 1
///       parameterValues:
///         connectionString: ${exampleNamespace.defaultPrimaryConnectionString}
///       tags:
///         Hello: World
/// variables:
///   example:
///     fn::invoke:
///       function: azure:connections:getManagedApi
///       arguments:
///         name: servicebus
///         location: ${exampleResourceGroup.location}
/// ```
///
/// ## Import
///
/// API Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:connections/apiConnection:ApiConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.Web/connections/example-connection
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiConnectionArgs {
        /// A display name for this API Connection. Defaults to `Service Bus`. Changing this forces a new API Connection to be created.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Managed API which this API Connection is linked to. Changing this forces a new API Connection to be created.
        #[builder(into)]
        pub managed_api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name which should be used for this API Connection. Changing this forces a new API Connection to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub parameter_values: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group where this API Connection should exist. Changing this forces a new API Connection to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the API Connection.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApiConnectionResult {
        /// A display name for this API Connection. Defaults to `Service Bus`. Changing this forces a new API Connection to be created.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Managed API which this API Connection is linked to. Changing this forces a new API Connection to be created.
        pub managed_api_id: pulumi_gestalt_rust::Output<String>,
        /// The Name which should be used for this API Connection. Changing this forces a new API Connection to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub parameter_values: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group where this API Connection should exist. Changing this forces a new API Connection to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the API Connection.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApiConnectionArgs,
    ) -> ApiConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let managed_api_id_binding_1 = args.managed_api_id.get_output(context);
        let managed_api_id_binding = managed_api_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parameter_values_binding_1 = args.parameter_values.get_output(context);
        let parameter_values_binding = parameter_values_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:connections/apiConnection:ApiConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "managedApiId".into(),
                    value: &managed_api_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameterValues".into(),
                    value: &parameter_values_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApiConnectionResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            managed_api_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedApiId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parameter_values: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameterValues"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}

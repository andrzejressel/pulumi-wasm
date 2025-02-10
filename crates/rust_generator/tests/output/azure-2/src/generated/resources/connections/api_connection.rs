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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiConnectionArgs,
    ) -> ApiConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let managed_api_id_binding = args.managed_api_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameter_values_binding = args.parameter_values.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:connections/apiConnection:ApiConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedApiId".into(),
                    value: managed_api_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameterValues".into(),
                    value: parameter_values_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiConnectionResult {
            display_name: o.get_field("displayName"),
            managed_api_id: o.get_field("managedApiId"),
            name: o.get_field("name"),
            parameter_values: o.get_field("parameterValues"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}

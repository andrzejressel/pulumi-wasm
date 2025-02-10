/// Manages an Azure Container Registry Webhook.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   acr:
///     type: azure:containerservice:Registry
///     properties:
///       name: containerRegistry1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: Standard
///       adminEnabled: false
///   webhook:
///     type: azure:containerservice:RegistryWebhook
///     properties:
///       name: mywebhook
///       resourceGroupName: ${example.name}
///       registryName: ${acr.name}
///       location: ${example.location}
///       serviceUri: https://mywebhookreceiver.example/mytag
///       status: enabled
///       scope: mytag:*
///       actions:
///         - push
///       customHeaders:
///         Content-Type: application/json
/// ```
///
/// ## Import
///
/// Container Registry Webhooks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/registryWebook:RegistryWebook example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ContainerRegistry/registries/myregistry1/webHooks/mywebhook1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod registry_webook {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryWebookArgs {
        /// A list of actions that trigger the Webhook to post notifications. At least one action needs to be specified. Valid values are: `push`, `delete`, `quarantine`, `chart_push`, `chart_delete`
        #[builder(into)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Custom headers that will be added to the webhook notifications request.
        #[builder(into, default)]
        pub custom_headers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Container Registry Webhook. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Name of Container registry this Webhook belongs to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub registry_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the Container Registry Webhook. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the scope of repositories that can trigger an event. For example, `foo:*` means events for all tags under repository `foo`. `foo:bar` means events for 'foo:bar' only. `foo` is equivalent to `foo:latest`. Empty means all events. Defaults to `""`.
        #[builder(into, default)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the service URI for the Webhook to post notifications.
        #[builder(into)]
        pub service_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies if this Webhook triggers notifications or not. Valid values: `enabled` and `disabled`. Default is `enabled`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegistryWebookResult {
        /// A list of actions that trigger the Webhook to post notifications. At least one action needs to be specified. Valid values are: `push`, `delete`, `quarantine`, `chart_push`, `chart_delete`
        pub actions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Custom headers that will be added to the webhook notifications request.
        pub custom_headers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Container Registry Webhook. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Name of Container registry this Webhook belongs to. Changing this forces a new resource to be created.
        pub registry_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Container Registry Webhook. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the scope of repositories that can trigger an event. For example, `foo:*` means events for all tags under repository `foo`. `foo:bar` means events for 'foo:bar' only. `foo` is equivalent to `foo:latest`. Empty means all events. Defaults to `""`.
        pub scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the service URI for the Webhook to post notifications.
        pub service_uri: pulumi_gestalt_rust::Output<String>,
        /// Specifies if this Webhook triggers notifications or not. Valid values: `enabled` and `disabled`. Default is `enabled`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
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
        args: RegistryWebookArgs,
    ) -> RegistryWebookResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let custom_headers_binding = args.custom_headers.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let registry_name_binding = args.registry_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let service_uri_binding = args.service_uri.get_output(context);
        let status_binding = args.status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/registryWebook:RegistryWebook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: actions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customHeaders".into(),
                    value: custom_headers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registryName".into(),
                    value: registry_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceUri".into(),
                    value: service_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegistryWebookResult {
            actions: o.get_field("actions"),
            custom_headers: o.get_field("customHeaders"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            registry_name: o.get_field("registryName"),
            resource_group_name: o.get_field("resourceGroupName"),
            scope: o.get_field("scope"),
            service_uri: o.get_field("serviceUri"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
        }
    }
}

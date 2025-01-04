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
/// $ pulumi import azure:containerservice/registryWebhook:RegistryWebhook example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ContainerRegistry/registries/myregistry1/webHooks/mywebhook1
/// ```
///
pub mod registry_webhook {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryWebhookArgs {
        /// A list of actions that trigger the Webhook to post notifications. At least one action needs to be specified. Valid values are: `push`, `delete`, `quarantine`, `chart_push`, `chart_delete`
        #[builder(into)]
        pub actions: pulumi_wasm_rust::Output<Vec<String>>,
        /// Custom headers that will be added to the webhook notifications request.
        #[builder(into, default)]
        pub custom_headers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Container Registry Webhook. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name of Container registry this Webhook belongs to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub registry_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Container Registry Webhook. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the scope of repositories that can trigger an event. For example, `foo:*` means events for all tags under repository `foo`. `foo:bar` means events for 'foo:bar' only. `foo` is equivalent to `foo:latest`. Empty means all events. Defaults to `""`.
        #[builder(into, default)]
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the service URI for the Webhook to post notifications.
        #[builder(into)]
        pub service_uri: pulumi_wasm_rust::Output<String>,
        /// Specifies if this Webhook triggers notifications or not. Valid values: `enabled` and `disabled`. Default is `enabled`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegistryWebhookResult {
        /// A list of actions that trigger the Webhook to post notifications. At least one action needs to be specified. Valid values are: `push`, `delete`, `quarantine`, `chart_push`, `chart_delete`
        pub actions: pulumi_wasm_rust::Output<Vec<String>>,
        /// Custom headers that will be added to the webhook notifications request.
        pub custom_headers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Container Registry Webhook. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of Container registry this Webhook belongs to. Changing this forces a new resource to be created.
        pub registry_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Container Registry Webhook. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the scope of repositories that can trigger an event. For example, `foo:*` means events for all tags under repository `foo`. `foo:bar` means events for 'foo:bar' only. `foo` is equivalent to `foo:latest`. Empty means all events. Defaults to `""`.
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the service URI for the Webhook to post notifications.
        pub service_uri: pulumi_wasm_rust::Output<String>,
        /// Specifies if this Webhook triggers notifications or not. Valid values: `enabled` and `disabled`. Default is `enabled`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RegistryWebhookArgs) -> RegistryWebhookResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_binding = args.actions.get_inner();
        let custom_headers_binding = args.custom_headers.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let registry_name_binding = args.registry_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let scope_binding = args.scope.get_inner();
        let service_uri_binding = args.service_uri.get_inner();
        let status_binding = args.status.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/registryWebhook:RegistryWebhook".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
                },
                register_interface::ObjectField {
                    name: "customHeaders".into(),
                    value: &custom_headers_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "registryName".into(),
                    value: &registry_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "serviceUri".into(),
                    value: &service_uri_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actions".into(),
                },
                register_interface::ResultField {
                    name: "customHeaders".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "registryName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "serviceUri".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegistryWebhookResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actions").unwrap(),
            ),
            custom_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customHeaders").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            registry_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            service_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceUri").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}

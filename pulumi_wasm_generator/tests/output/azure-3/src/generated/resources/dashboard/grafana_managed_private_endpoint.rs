/// Manages a Dashboard Grafana Managed Private Endpoint.
///
/// > **NOTE:** This resource will _not_ approve the managed private endpoint connection on the linked resource. This will need to be done manually via Azure CLI, PowerShell, or AzAPI resources. See here for an example that uses AzAPI.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: Canada Central
///   exampleWorkspace:
///     type: azure:monitoring:Workspace
///     name: example
///     properties:
///       name: example-mamw
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       publicNetworkAccessEnabled: false
///   exampleGrafana:
///     type: azure:dashboard:Grafana
///     name: example
///     properties:
///       name: example-dg
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       grafanaMajorVersion: 10
///       publicNetworkAccessEnabled: false
///       azureMonitorWorkspaceIntegrations:
///         - resourceId: ${exampleWorkspace.id}
///   exampleGrafanaManagedPrivateEndpoint:
///     type: azure:dashboard:GrafanaManagedPrivateEndpoint
///     name: example
///     properties:
///       grafanaId: ${exampleGrafana.id}
///       name: example-mpe
///       location: ${exampleGrafana.location}
///       privateLinkResourceId: ${exampleWorkspace.id}
///       groupIds:
///         - prometheusMetrics
///       privateLinkResourceRegion: ${exampleGrafana.location}
/// ```
///
/// ## Import
///
/// Dashboard Grafana Managed Private Endpoint Examples can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dashboard/grafanaManagedPrivateEndpoint:GrafanaManagedPrivateEndpoint example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Dashboard/grafana/workspace1/managedPrivateEndpoints/endpoint1
/// ```
///
pub mod grafana_managed_private_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GrafanaManagedPrivateEndpointArgs {
        /// The id of the associated managed Grafana. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        #[builder(into)]
        pub grafana_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies a list of private link group IDs. The value of this will depend on the private link resource to which you are connecting. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        #[builder(into, default)]
        pub group_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Azure Region where the Dashboard Grafana Managed Private Endpoint should exist. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Dashboard Grafana Managed Private Endpoint. Must be between 2 and 20 alphanumeric characters or dashes, must begin with letter and end with a letter or number. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the resource to which this Dashboard Grafana Managed Private Endpoint will connect. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        #[builder(into)]
        pub private_link_resource_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The region in which to create the private link. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        #[builder(into, default)]
        pub private_link_resource_region: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// A message to provide in the request which will be seen by approvers.
        #[builder(into, default)]
        pub request_message: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Dashboard Grafana Managed Private Endpoint.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GrafanaManagedPrivateEndpointResult {
        /// The id of the associated managed Grafana. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        pub grafana_id: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of private link group IDs. The value of this will depend on the private link resource to which you are connecting. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        pub group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Azure Region where the Dashboard Grafana Managed Private Endpoint should exist. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Dashboard Grafana Managed Private Endpoint. Must be between 2 and 20 alphanumeric characters or dashes, must begin with letter and end with a letter or number. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the resource to which this Dashboard Grafana Managed Private Endpoint will connect. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        pub private_link_resource_id: pulumi_wasm_rust::Output<String>,
        /// The region in which to create the private link. Changing this forces a new Dashboard Grafana Managed Private Endpoint to be created.
        pub private_link_resource_region: pulumi_wasm_rust::Output<Option<String>>,
        /// A message to provide in the request which will be seen by approvers.
        pub request_message: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Dashboard Grafana Managed Private Endpoint.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GrafanaManagedPrivateEndpointArgs,
    ) -> GrafanaManagedPrivateEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let grafana_id_binding = args.grafana_id.get_output(context).get_inner();
        let group_ids_binding = args.group_ids.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let private_link_resource_id_binding = args
            .private_link_resource_id
            .get_output(context)
            .get_inner();
        let private_link_resource_region_binding = args
            .private_link_resource_region
            .get_output(context)
            .get_inner();
        let request_message_binding = args
            .request_message
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dashboard/grafanaManagedPrivateEndpoint:GrafanaManagedPrivateEndpoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "grafanaId".into(),
                    value: &grafana_id_binding,
                },
                register_interface::ObjectField {
                    name: "groupIds".into(),
                    value: &group_ids_binding,
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
                    name: "privateLinkResourceId".into(),
                    value: &private_link_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "privateLinkResourceRegion".into(),
                    value: &private_link_resource_region_binding,
                },
                register_interface::ObjectField {
                    name: "requestMessage".into(),
                    value: &request_message_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "grafanaId".into(),
                },
                register_interface::ResultField {
                    name: "groupIds".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateLinkResourceId".into(),
                },
                register_interface::ResultField {
                    name: "privateLinkResourceRegion".into(),
                },
                register_interface::ResultField {
                    name: "requestMessage".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GrafanaManagedPrivateEndpointResult {
            grafana_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grafanaId").unwrap(),
            ),
            group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupIds").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_link_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateLinkResourceId").unwrap(),
            ),
            private_link_resource_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateLinkResourceRegion").unwrap(),
            ),
            request_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestMessage").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}

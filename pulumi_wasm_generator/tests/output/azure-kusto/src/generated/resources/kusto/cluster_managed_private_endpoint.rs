/// Manages a Managed Private Endpoint for a Kusto Cluster.
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
///   exampleCluster:
///     type: azure:kusto:Cluster
///     name: example
///     properties:
///       name: examplekc
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         name: Dev(No SLA)_Standard_D11_v2
///         capacity: 1
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleClusterManagedPrivateEndpoint:
///     type: azure:kusto:ClusterManagedPrivateEndpoint
///     name: example
///     properties:
///       name: examplempe
///       resourceGroupName: ${example.name}
///       clusterName: ${exampleCluster.name}
///       privateLinkResourceId: ${exampleAccount.id}
///       privateLinkResourceRegion: ${exampleAccount.location}
///       groupId: blob
///       requestMessage: Please Approve
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Managed Private Endpoint for a Kusto Cluster can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/clusterManagedPrivateEndpoint:ClusterManagedPrivateEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1/managedPrivateEndpoints/managedPrivateEndpoint1
/// ```
///
pub mod cluster_managed_private_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterManagedPrivateEndpointArgs {
        /// The name of the Kusto Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The group id in which the managed private endpoint is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub group_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Managed Private Endpoints to create. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARM resource ID of the resource for which the managed private endpoint is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub private_link_resource_id: pulumi_wasm_rust::Output<String>,
        /// The region of the resource to which the managed private endpoint is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub private_link_resource_region: pulumi_wasm_rust::Output<Option<String>>,
        /// The user request message.
        #[builder(into, default)]
        pub request_message: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Resource Group where the Kusto Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterManagedPrivateEndpointResult {
        /// The name of the Kusto Cluster. Changing this forces a new resource to be created.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The group id in which the managed private endpoint is created. Changing this forces a new resource to be created.
        pub group_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Managed Private Endpoints to create. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ARM resource ID of the resource for which the managed private endpoint is created. Changing this forces a new resource to be created.
        pub private_link_resource_id: pulumi_wasm_rust::Output<String>,
        /// The region of the resource to which the managed private endpoint is created. Changing this forces a new resource to be created.
        pub private_link_resource_region: pulumi_wasm_rust::Output<Option<String>>,
        /// The user request message.
        pub request_message: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Resource Group where the Kusto Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ClusterManagedPrivateEndpointArgs,
    ) -> ClusterManagedPrivateEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_inner();
        let group_id_binding = args.group_id.get_inner();
        let name_binding = args.name.get_inner();
        let private_link_resource_id_binding = args.private_link_resource_id.get_inner();
        let private_link_resource_region_binding = args
            .private_link_resource_region
            .get_inner();
        let request_message_binding = args.request_message.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:kusto/clusterManagedPrivateEndpoint:ClusterManagedPrivateEndpoint"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "groupId".into(),
                    value: &group_id_binding,
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterName".into(),
                },
                register_interface::ResultField {
                    name: "groupId".into(),
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
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterManagedPrivateEndpointResult {
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterName").unwrap(),
            ),
            group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupId").unwrap(),
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
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
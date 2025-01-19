/// Manages a Databricks Virtual Network Peering
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
///   remote:
///     type: azure:network:VirtualNetwork
///     properties:
///       name: remote-vnet
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.1.0/24
///       location: ${example.location}
///   exampleWorkspace:
///     type: azure:databricks:Workspace
///     name: example
///     properties:
///       name: example-workspace
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: standard
///   exampleVirtualNetworkPeering:
///     type: azure:databricks:VirtualNetworkPeering
///     name: example
///     properties:
///       name: databricks-vnet-peer
///       resourceGroupName: ${example.name}
///       workspaceId: ${exampleWorkspace.id}
///       remoteAddressSpacePrefixes: ${remote.addressSpaces}
///       remoteVirtualNetworkId: ${remote.id}
///       allowVirtualNetworkAccess: true
///   remoteVirtualNetworkPeering:
///     type: azure:network:VirtualNetworkPeering
///     name: remote
///     properties:
///       name: peer-to-databricks
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${remote.name}
///       remoteVirtualNetworkId: ${exampleVirtualNetworkPeering.virtualNetworkId}
///       allowVirtualNetworkAccess: true
/// ```
///
/// ## Import
///
/// Databrick Virtual Network Peerings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:databricks/virtualNetworkPeering:VirtualNetworkPeering example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Databricks/workspaces/workspace1/virtualNetworkPeerings/peering1
/// ```
///
pub mod virtual_network_peering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkPeeringArgs {
        /// Can the forwarded traffic from the VMs in the local virtual network be forwarded to the remote virtual network? Defaults to `false`.
        #[builder(into, default)]
        pub allow_forwarded_traffic: pulumi_wasm_rust::Output<Option<bool>>,
        /// Can the gateway links be used in the remote virtual network to link to the Databricks virtual network? Defaults to `false`.
        #[builder(into, default)]
        pub allow_gateway_transit: pulumi_wasm_rust::Output<Option<bool>>,
        /// Can the VMs in the local virtual network space access the VMs in the remote virtual network space? Defaults to `true`.
        #[builder(into, default)]
        pub allow_virtual_network_access: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Databricks Virtual Network Peering resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of address blocks reserved for the remote virtual network in CIDR notation. Changing this forces a new resource to be created.
        #[builder(into)]
        pub remote_address_space_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the remote virtual network. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The remote virtual network should be in the same region as the databricks workspace. Please see the [product documentation](https://learn.microsoft.com/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering) for more information.
        #[builder(into)]
        pub remote_virtual_network_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Databricks Virtual Network Peering should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Can remote gateways be used on the Databricks virtual network? Defaults to `false`.
        ///
        /// > **NOTE:** If the `use_remote_gateways` is set to `true`, and `allow_gateway_transit` on the remote peering is also `true`, the virtual network will use the gateways of the remote virtual network for transit. Only one peering can have this flag set to `true`. `use_remote_gateways` cannot be set if the virtual network already has a gateway.
        #[builder(into, default)]
        pub use_remote_gateways: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Databricks Workspace that this Databricks Virtual Network Peering is bound. Changing this forces a new resource to be created.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkPeeringResult {
        /// A list of address blocks reserved for this virtual network in CIDR notation.
        pub address_space_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        /// Can the forwarded traffic from the VMs in the local virtual network be forwarded to the remote virtual network? Defaults to `false`.
        pub allow_forwarded_traffic: pulumi_wasm_rust::Output<Option<bool>>,
        /// Can the gateway links be used in the remote virtual network to link to the Databricks virtual network? Defaults to `false`.
        pub allow_gateway_transit: pulumi_wasm_rust::Output<Option<bool>>,
        /// Can the VMs in the local virtual network space access the VMs in the remote virtual network space? Defaults to `true`.
        pub allow_virtual_network_access: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Databricks Virtual Network Peering resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of address blocks reserved for the remote virtual network in CIDR notation. Changing this forces a new resource to be created.
        pub remote_address_space_prefixes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the remote virtual network. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The remote virtual network should be in the same region as the databricks workspace. Please see the [product documentation](https://learn.microsoft.com/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering) for more information.
        pub remote_virtual_network_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Databricks Virtual Network Peering should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Can remote gateways be used on the Databricks virtual network? Defaults to `false`.
        ///
        /// > **NOTE:** If the `use_remote_gateways` is set to `true`, and `allow_gateway_transit` on the remote peering is also `true`, the virtual network will use the gateways of the remote virtual network for transit. Only one peering can have this flag set to `true`. `use_remote_gateways` cannot be set if the virtual network already has a gateway.
        pub use_remote_gateways: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the internal Virtual Network used by the DataBricks Workspace.
        ///
        /// > **NOTE:** The `virtual_network_id` field is the value you must supply to the `azure.network.VirtualNetworkPeering` resources `remote_virtual_network_id` field to successfully peer the Databricks Virtual Network with the remote virtual network.
        pub virtual_network_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Databricks Workspace that this Databricks Virtual Network Peering is bound. Changing this forces a new resource to be created.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VirtualNetworkPeeringArgs,
    ) -> VirtualNetworkPeeringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_forwarded_traffic_binding = args.allow_forwarded_traffic.get_inner();
        let allow_gateway_transit_binding = args.allow_gateway_transit.get_inner();
        let allow_virtual_network_access_binding = args
            .allow_virtual_network_access
            .get_inner();
        let name_binding = args.name.get_inner();
        let remote_address_space_prefixes_binding = args
            .remote_address_space_prefixes
            .get_inner();
        let remote_virtual_network_id_binding = args
            .remote_virtual_network_id
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let use_remote_gateways_binding = args.use_remote_gateways.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:databricks/virtualNetworkPeering:VirtualNetworkPeering".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowForwardedTraffic".into(),
                    value: &allow_forwarded_traffic_binding,
                },
                register_interface::ObjectField {
                    name: "allowGatewayTransit".into(),
                    value: &allow_gateway_transit_binding,
                },
                register_interface::ObjectField {
                    name: "allowVirtualNetworkAccess".into(),
                    value: &allow_virtual_network_access_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "remoteAddressSpacePrefixes".into(),
                    value: &remote_address_space_prefixes_binding,
                },
                register_interface::ObjectField {
                    name: "remoteVirtualNetworkId".into(),
                    value: &remote_virtual_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "useRemoteGateways".into(),
                    value: &use_remote_gateways_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressSpacePrefixes".into(),
                },
                register_interface::ResultField {
                    name: "allowForwardedTraffic".into(),
                },
                register_interface::ResultField {
                    name: "allowGatewayTransit".into(),
                },
                register_interface::ResultField {
                    name: "allowVirtualNetworkAccess".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "remoteAddressSpacePrefixes".into(),
                },
                register_interface::ResultField {
                    name: "remoteVirtualNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "useRemoteGateways".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualNetworkPeeringResult {
            address_space_prefixes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressSpacePrefixes").unwrap(),
            ),
            allow_forwarded_traffic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowForwardedTraffic").unwrap(),
            ),
            allow_gateway_transit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowGatewayTransit").unwrap(),
            ),
            allow_virtual_network_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowVirtualNetworkAccess").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            remote_address_space_prefixes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteAddressSpacePrefixes").unwrap(),
            ),
            remote_virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteVirtualNetworkId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            use_remote_gateways: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useRemoteGateways").unwrap(),
            ),
            virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkId").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}

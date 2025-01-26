/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       subnets:
///         - name: subnet1
///           addressPrefix: 10.0.1.0/24
///   exampleVirtualNetworkDnsServers:
///     type: azure:network:VirtualNetworkDnsServers
///     name: example
///     properties:
///       virtualNetworkId: ${exampleVirtualNetwork.id}
///       dnsServers:
///         - 10.7.7.2
///         - 10.7.7.7
///         - 10.7.7.1
/// ```
///
/// ## Import
///
/// Virtual Network DNS Servers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualNetworkDnsServers:VirtualNetworkDnsServers exampleNetwork /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/virtualNetworks/myvnet1/dnsServers/default
/// ```
///
pub mod virtual_network_dns_servers {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkDnsServersArgs {
        /// List of IP addresses of DNS servers
        #[builder(into, default)]
        pub dns_servers: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the Virtual Network that should be linked to the DNS Zone. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_network_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkDnsServersResult {
        /// List of IP addresses of DNS servers
        pub dns_servers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the Virtual Network that should be linked to the DNS Zone. Changing this forces a new resource to be created.
        pub virtual_network_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VirtualNetworkDnsServersArgs,
    ) -> VirtualNetworkDnsServersResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dns_servers_binding = args.dns_servers.get_output(context).get_inner();
        let virtual_network_id_binding = args
            .virtual_network_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/virtualNetworkDnsServers:VirtualNetworkDnsServers"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dnsServers".into(),
                    value: &dns_servers_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VirtualNetworkDnsServersResult {
            dns_servers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsServers"),
            ),
            virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualNetworkId"),
            ),
        }
    }
}

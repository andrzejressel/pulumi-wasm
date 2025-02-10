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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_network_dns_servers {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkDnsServersArgs {
        /// List of IP addresses of DNS servers
        #[builder(into, default)]
        pub dns_servers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the Virtual Network that should be linked to the DNS Zone. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkDnsServersResult {
        /// List of IP addresses of DNS servers
        pub dns_servers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the Virtual Network that should be linked to the DNS Zone. Changing this forces a new resource to be created.
        pub virtual_network_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualNetworkDnsServersArgs,
    ) -> VirtualNetworkDnsServersResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dns_servers_binding = args.dns_servers.get_output(context);
        let virtual_network_id_binding = args.virtual_network_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/virtualNetworkDnsServers:VirtualNetworkDnsServers"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsServers".into(),
                    value: dns_servers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkId".into(),
                    value: virtual_network_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualNetworkDnsServersResult {
            dns_servers: o.get_field("dnsServers"),
            virtual_network_id: o.get_field("virtualNetworkId"),
        }
    }
}

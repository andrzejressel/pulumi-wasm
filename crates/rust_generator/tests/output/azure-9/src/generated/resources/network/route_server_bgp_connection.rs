/// Manages a Bgp Connection for a Route Server
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
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vn
///       addressSpaces:
///         - 10.0.0.0/16
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       tags:
///         environment: Production
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: RouteServerSubnet
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       resourceGroupName: ${example.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example-pip
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       allocationMethod: Static
///       sku: Standard
///   exampleRouteServer:
///     type: azure:network:RouteServer
///     name: example
///     properties:
///       name: example-routerserver
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: Standard
///       publicIpAddressId: ${examplePublicIp.id}
///       subnetId: ${exampleSubnet.id}
///       branchToBranchTrafficEnabled: true
///   exampleRouteServerBgpConnection:
///     type: azure:network:RouteServerBgpConnection
///     name: example
///     properties:
///       name: example-rs-bgpconnection
///       routeServerId: ${exampleRouteServer.id}
///       peerAsn: 65501
///       peerIp: 169.254.21.5
/// ```
///
/// ## Import
///
/// Route Server Bgp Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/routeServerBgpConnection:RouteServerBgpConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/virtualHubs/routeServer1/bgpConnections/connection1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route_server_bgp_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteServerBgpConnectionArgs {
        /// The name which should be used for this Route Server Bgp Connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The peer autonomous system number for the Route Server Bgp Connection. Changing this forces a new resource to be created.
        #[builder(into)]
        pub peer_asn: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The peer ip address for the Route Server Bgp Connection. Changing this forces a new resource to be created.
        #[builder(into)]
        pub peer_ip: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Route Server within which this Bgp connection should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub route_server_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteServerBgpConnectionResult {
        /// The name which should be used for this Route Server Bgp Connection. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The peer autonomous system number for the Route Server Bgp Connection. Changing this forces a new resource to be created.
        pub peer_asn: pulumi_gestalt_rust::Output<i32>,
        /// The peer ip address for the Route Server Bgp Connection. Changing this forces a new resource to be created.
        pub peer_ip: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Route Server within which this Bgp connection should be created. Changing this forces a new resource to be created.
        pub route_server_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteServerBgpConnectionArgs,
    ) -> RouteServerBgpConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let peer_asn_binding = args.peer_asn.get_output(context);
        let peer_ip_binding = args.peer_ip.get_output(context);
        let route_server_id_binding = args.route_server_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/routeServerBgpConnection:RouteServerBgpConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerAsn".into(),
                    value: peer_asn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerIp".into(),
                    value: peer_ip_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeServerId".into(),
                    value: route_server_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouteServerBgpConnectionResult {
            name: o.get_field("name"),
            peer_asn: o.get_field("peerAsn"),
            peer_ip: o.get_field("peerIp"),
            route_server_id: o.get_field("routeServerId"),
        }
    }
}

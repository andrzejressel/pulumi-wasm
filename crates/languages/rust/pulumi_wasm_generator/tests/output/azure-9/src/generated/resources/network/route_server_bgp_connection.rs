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
pub mod route_server_bgp_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteServerBgpConnectionArgs {
        /// The name which should be used for this Route Server Bgp Connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The peer autonomous system number for the Route Server Bgp Connection. Changing this forces a new resource to be created.
        #[builder(into)]
        pub peer_asn: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The peer ip address for the Route Server Bgp Connection. Changing this forces a new resource to be created.
        #[builder(into)]
        pub peer_ip: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Route Server within which this Bgp connection should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub route_server_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteServerBgpConnectionResult {
        /// The name which should be used for this Route Server Bgp Connection. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The peer autonomous system number for the Route Server Bgp Connection. Changing this forces a new resource to be created.
        pub peer_asn: pulumi_wasm_rust::Output<i32>,
        /// The peer ip address for the Route Server Bgp Connection. Changing this forces a new resource to be created.
        pub peer_ip: pulumi_wasm_rust::Output<String>,
        /// The ID of the Route Server within which this Bgp connection should be created. Changing this forces a new resource to be created.
        pub route_server_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RouteServerBgpConnectionArgs,
    ) -> RouteServerBgpConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let peer_asn_binding = args.peer_asn.get_output(context).get_inner();
        let peer_ip_binding = args.peer_ip.get_output(context).get_inner();
        let route_server_id_binding = args
            .route_server_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/routeServerBgpConnection:RouteServerBgpConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "peerAsn".into(),
                    value: &peer_asn_binding,
                },
                register_interface::ObjectField {
                    name: "peerIp".into(),
                    value: &peer_ip_binding,
                },
                register_interface::ObjectField {
                    name: "routeServerId".into(),
                    value: &route_server_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RouteServerBgpConnectionResult {
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            peer_asn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peerAsn"),
            ),
            peer_ip: pulumi_wasm_rust::__private::into_domain(o.extract_field("peerIp")),
            route_server_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("routeServerId"),
            ),
        }
    }
}

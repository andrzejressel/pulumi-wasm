/// Manages a Bgp Connection for a Virtual Hub.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleBgpConnection = bgp_connection::create(
///         "exampleBgpConnection",
///         BgpConnectionArgs::builder()
///             .name("example-vhub-bgpconnection")
///             .peer_asn(65514)
///             .peer_ip("169.254.21.5")
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("example-pip")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.5.1.0/24",])
///             .name("RouteServerSubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .location("${example.location}")
///             .name("example-vhub")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleVirtualHubIp = virtual_hub_ip::create(
///         "exampleVirtualHubIp",
///         VirtualHubIpArgs::builder()
///             .name("example-vhubip")
///             .private_ip_address("10.5.1.18")
///             .private_ip_allocation_method("Static")
///             .public_ip_address_id("${examplePublicIp.id}")
///             .subnet_id("${exampleSubnet.id}")
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.5.0.0/16",])
///             .location("${example.location}")
///             .name("example-vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Hub Bgp Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/bgpConnection:BgpConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/virtualHubs/virtualHub1/bgpConnections/connection1
/// ```
///
pub mod bgp_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BgpConnectionArgs {
        /// The name which should be used for this Virtual Hub Bgp Connection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The peer autonomous system number for the Virtual Hub Bgp Connection. Changing this forces a new resource to be created.
        #[builder(into)]
        pub peer_asn: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The peer IP address for the Virtual Hub Bgp Connection. Changing this forces a new resource to be created.
        #[builder(into)]
        pub peer_ip: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Virtual Hub within which this Bgp connection should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of virtual network connection.
        #[builder(into, default)]
        pub virtual_network_connection_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct BgpConnectionResult {
        /// The name which should be used for this Virtual Hub Bgp Connection. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The peer autonomous system number for the Virtual Hub Bgp Connection. Changing this forces a new resource to be created.
        pub peer_asn: pulumi_wasm_rust::Output<i32>,
        /// The peer IP address for the Virtual Hub Bgp Connection. Changing this forces a new resource to be created.
        pub peer_ip: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Hub within which this Bgp connection should be created. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_wasm_rust::Output<String>,
        /// The ID of virtual network connection.
        pub virtual_network_connection_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BgpConnectionArgs,
    ) -> BgpConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let peer_asn_binding = args.peer_asn.get_output(context).get_inner();
        let peer_ip_binding = args.peer_ip.get_output(context).get_inner();
        let virtual_hub_id_binding = args.virtual_hub_id.get_output(context).get_inner();
        let virtual_network_connection_id_binding = args
            .virtual_network_connection_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/bgpConnection:BgpConnection".into(),
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
                    name: "virtualHubId".into(),
                    value: &virtual_hub_id_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkConnectionId".into(),
                    value: &virtual_network_connection_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "peerAsn".into(),
                },
                register_interface::ResultField {
                    name: "peerIp".into(),
                },
                register_interface::ResultField {
                    name: "virtualHubId".into(),
                },
                register_interface::ResultField {
                    name: "virtualNetworkConnectionId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BgpConnectionResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            peer_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerAsn").unwrap(),
            ),
            peer_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerIp").unwrap(),
            ),
            virtual_hub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHubId").unwrap(),
            ),
            virtual_network_connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualNetworkConnectionId").unwrap(),
            ),
        }
    }
}

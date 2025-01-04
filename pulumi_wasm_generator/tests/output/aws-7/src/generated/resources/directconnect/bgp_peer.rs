/// Provides a Direct Connect BGP peer resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let peer = bgp_peer::create(
///         "peer",
///         BgpPeerArgs::builder()
///             .address_family("ipv6")
///             .bgp_asn(65351)
///             .virtual_interface_id("${foo.id}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod bgp_peer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BgpPeerArgs {
        /// The address family for the BGP peer. `ipv4 ` or `ipv6`.
        #[builder(into)]
        pub address_family: pulumi_wasm_rust::Output<String>,
        /// The IPv4 CIDR address to use to send traffic to Amazon.
        /// Required for IPv4 BGP peers on public virtual interfaces.
        #[builder(into, default)]
        pub amazon_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.
        #[builder(into)]
        pub bgp_asn: pulumi_wasm_rust::Output<i32>,
        /// The authentication key for BGP configuration.
        #[builder(into, default)]
        pub bgp_auth_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 CIDR destination address to which Amazon should send traffic.
        /// Required for IPv4 BGP peers on public virtual interfaces.
        #[builder(into, default)]
        pub customer_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Direct Connect virtual interface on which to create the BGP peer.
        #[builder(into)]
        pub virtual_interface_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BgpPeerResult {
        /// The address family for the BGP peer. `ipv4 ` or `ipv6`.
        pub address_family: pulumi_wasm_rust::Output<String>,
        /// The IPv4 CIDR address to use to send traffic to Amazon.
        /// Required for IPv4 BGP peers on public virtual interfaces.
        pub amazon_address: pulumi_wasm_rust::Output<String>,
        /// The Direct Connect endpoint on which the BGP peer terminates.
        pub aws_device: pulumi_wasm_rust::Output<String>,
        /// The autonomous system (AS) number for Border Gateway Protocol (BGP) configuration.
        pub bgp_asn: pulumi_wasm_rust::Output<i32>,
        /// The authentication key for BGP configuration.
        pub bgp_auth_key: pulumi_wasm_rust::Output<String>,
        /// The ID of the BGP peer.
        pub bgp_peer_id: pulumi_wasm_rust::Output<String>,
        /// The Up/Down state of the BGP peer.
        pub bgp_status: pulumi_wasm_rust::Output<String>,
        /// The IPv4 CIDR destination address to which Amazon should send traffic.
        /// Required for IPv4 BGP peers on public virtual interfaces.
        pub customer_address: pulumi_wasm_rust::Output<String>,
        /// The ID of the Direct Connect virtual interface on which to create the BGP peer.
        pub virtual_interface_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BgpPeerArgs) -> BgpPeerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_family_binding = args.address_family.get_inner();
        let amazon_address_binding = args.amazon_address.get_inner();
        let bgp_asn_binding = args.bgp_asn.get_inner();
        let bgp_auth_key_binding = args.bgp_auth_key.get_inner();
        let customer_address_binding = args.customer_address.get_inner();
        let virtual_interface_id_binding = args.virtual_interface_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/bgpPeer:BgpPeer".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addressFamily".into(),
                    value: &address_family_binding,
                },
                register_interface::ObjectField {
                    name: "amazonAddress".into(),
                    value: &amazon_address_binding,
                },
                register_interface::ObjectField {
                    name: "bgpAsn".into(),
                    value: &bgp_asn_binding,
                },
                register_interface::ObjectField {
                    name: "bgpAuthKey".into(),
                    value: &bgp_auth_key_binding,
                },
                register_interface::ObjectField {
                    name: "customerAddress".into(),
                    value: &customer_address_binding,
                },
                register_interface::ObjectField {
                    name: "virtualInterfaceId".into(),
                    value: &virtual_interface_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressFamily".into(),
                },
                register_interface::ResultField {
                    name: "amazonAddress".into(),
                },
                register_interface::ResultField {
                    name: "awsDevice".into(),
                },
                register_interface::ResultField {
                    name: "bgpAsn".into(),
                },
                register_interface::ResultField {
                    name: "bgpAuthKey".into(),
                },
                register_interface::ResultField {
                    name: "bgpPeerId".into(),
                },
                register_interface::ResultField {
                    name: "bgpStatus".into(),
                },
                register_interface::ResultField {
                    name: "customerAddress".into(),
                },
                register_interface::ResultField {
                    name: "virtualInterfaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BgpPeerResult {
            address_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressFamily").unwrap(),
            ),
            amazon_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("amazonAddress").unwrap(),
            ),
            aws_device: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsDevice").unwrap(),
            ),
            bgp_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpAsn").unwrap(),
            ),
            bgp_auth_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpAuthKey").unwrap(),
            ),
            bgp_peer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpPeerId").unwrap(),
            ),
            bgp_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpStatus").unwrap(),
            ),
            customer_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerAddress").unwrap(),
            ),
            virtual_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualInterfaceId").unwrap(),
            ),
        }
    }
}

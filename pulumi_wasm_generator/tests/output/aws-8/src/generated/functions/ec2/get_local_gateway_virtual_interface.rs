pub mod get_local_gateway_virtual_interface {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalGatewayVirtualInterfaceArgs {
        /// One or more configuration blocks containing name-values filters. See the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeLocalGatewayVirtualInterfaces.html) for supported filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2::GetLocalGatewayVirtualInterfaceFilter,
                >,
            >,
        >,
        /// Identifier of EC2 Local Gateway Virtual Interface.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags, each pair of which must exactly match a pair on the desired local gateway route table.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLocalGatewayVirtualInterfaceResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2::GetLocalGatewayVirtualInterfaceFilter,
                >,
            >,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        /// Local address.
        pub local_address: pulumi_wasm_rust::Output<String>,
        /// Border Gateway Protocol (BGP) Autonomous System Number (ASN) of the EC2 Local Gateway.
        pub local_bgp_asn: pulumi_wasm_rust::Output<i32>,
        /// Identifier of the EC2 Local Gateway.
        pub local_gateway_id: pulumi_wasm_rust::Output<String>,
        pub local_gateway_virtual_interface_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Peer address.
        pub peer_address: pulumi_wasm_rust::Output<String>,
        /// Border Gateway Protocol (BGP) Autonomous System Number (ASN) of the peer.
        pub peer_bgp_asn: pulumi_wasm_rust::Output<i32>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Virtual Local Area Network.
        pub vlan: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetLocalGatewayVirtualInterfaceArgs,
    ) -> GetLocalGatewayVirtualInterfaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let id_binding = args.id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getLocalGatewayVirtualInterface:getLocalGatewayVirtualInterface"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "localAddress".into(),
                },
                register_interface::ResultField {
                    name: "localBgpAsn".into(),
                },
                register_interface::ResultField {
                    name: "localGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "localGatewayVirtualInterfaceIds".into(),
                },
                register_interface::ResultField {
                    name: "peerAddress".into(),
                },
                register_interface::ResultField {
                    name: "peerBgpAsn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vlan".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLocalGatewayVirtualInterfaceResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            local_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAddress").unwrap(),
            ),
            local_bgp_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localBgpAsn").unwrap(),
            ),
            local_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localGatewayId").unwrap(),
            ),
            local_gateway_virtual_interface_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localGatewayVirtualInterfaceIds").unwrap(),
            ),
            peer_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerAddress").unwrap(),
            ),
            peer_bgp_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerBgpAsn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vlan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vlan").unwrap(),
            ),
        }
    }
}

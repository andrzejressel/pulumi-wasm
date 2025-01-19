pub mod get_connect_peer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectPeerArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetConnectPeerFilter>,
            >,
        >,
        /// Key-value tags for the EC2 Transit Gateway Connect Peer
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the EC2 Transit Gateway Connect Peer.
        #[builder(into, default)]
        pub transit_gateway_connect_peer_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetConnectPeerResult {
        /// EC2 Transit Gateway Connect Peer ARN
        pub arn: pulumi_wasm_rust::Output<String>,
        /// BGP ASN number assigned customer device
        pub bgp_asn: pulumi_wasm_rust::Output<String>,
        /// The IP address assigned to customer device, which is used as BGP IP address.
        pub bgp_peer_address: pulumi_wasm_rust::Output<String>,
        /// The IP addresses assigned to Transit Gateway, which are used as BGP IP addresses.
        pub bgp_transit_gateway_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetConnectPeerFilter>,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// CIDR blocks that will be used for addressing within the tunnel.
        pub inside_cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        /// IP addressed assigned to customer device, which is used as tunnel endpoint
        pub peer_address: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Connect Peer
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The IP address assigned to Transit Gateway, which is used as tunnel endpoint.
        pub transit_gateway_address: pulumi_wasm_rust::Output<String>,
        /// The Transit Gateway Connect
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
        pub transit_gateway_connect_peer_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetConnectPeerArgs) -> GetConnectPeerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let tags_binding = args.tags.get_inner();
        let transit_gateway_connect_peer_id_binding = args
            .transit_gateway_connect_peer_id
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2transitgateway/getConnectPeer:getConnectPeer".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayConnectPeerId".into(),
                    value: &transit_gateway_connect_peer_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "bgpAsn".into(),
                },
                register_interface::ResultField {
                    name: "bgpPeerAddress".into(),
                },
                register_interface::ResultField {
                    name: "bgpTransitGatewayAddresses".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "insideCidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "peerAddress".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAddress".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAttachmentId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayConnectPeerId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetConnectPeerResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            bgp_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpAsn").unwrap(),
            ),
            bgp_peer_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpPeerAddress").unwrap(),
            ),
            bgp_transit_gateway_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgpTransitGatewayAddresses").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            inside_cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("insideCidrBlocks").unwrap(),
            ),
            peer_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerAddress").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            transit_gateway_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAddress").unwrap(),
            ),
            transit_gateway_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAttachmentId").unwrap(),
            ),
            transit_gateway_connect_peer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayConnectPeerId").unwrap(),
            ),
        }
    }
}

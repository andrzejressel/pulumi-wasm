pub mod get_local_gateway_virtual_interface {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalGatewayVirtualInterfaceArgs {
        /// One or more configuration blocks containing name-values filters. See the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeLocalGatewayVirtualInterfaces.html) for supported filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2::GetLocalGatewayVirtualInterfaceFilter,
                >,
            >,
        >,
        /// Identifier of EC2 Local Gateway Virtual Interface.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags, each pair of which must exactly match a pair on the desired local gateway route table.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLocalGatewayVirtualInterfaceArgs,
    ) -> GetLocalGatewayVirtualInterfaceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLocalGatewayVirtualInterfaceResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            local_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("localAddress"),
            ),
            local_bgp_asn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("localBgpAsn"),
            ),
            local_gateway_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("localGatewayId"),
            ),
            local_gateway_virtual_interface_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("localGatewayVirtualInterfaceIds"),
            ),
            peer_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peerAddress"),
            ),
            peer_bgp_asn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("peerBgpAsn"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            vlan: pulumi_wasm_rust::__private::into_domain(o.extract_field("vlan")),
        }
    }
}

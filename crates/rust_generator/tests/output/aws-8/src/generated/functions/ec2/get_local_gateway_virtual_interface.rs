#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_local_gateway_virtual_interface {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalGatewayVirtualInterfaceArgs {
        /// One or more configuration blocks containing name-values filters. See the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeLocalGatewayVirtualInterfaces.html) for supported filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2::GetLocalGatewayVirtualInterfaceFilter,
                >,
            >,
        >,
        /// Identifier of EC2 Local Gateway Virtual Interface.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags, each pair of which must exactly match a pair on the desired local gateway route table.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLocalGatewayVirtualInterfaceResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2::GetLocalGatewayVirtualInterfaceFilter,
                >,
            >,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Local address.
        pub local_address: pulumi_gestalt_rust::Output<String>,
        /// Border Gateway Protocol (BGP) Autonomous System Number (ASN) of the EC2 Local Gateway.
        pub local_bgp_asn: pulumi_gestalt_rust::Output<i32>,
        /// Identifier of the EC2 Local Gateway.
        pub local_gateway_id: pulumi_gestalt_rust::Output<String>,
        pub local_gateway_virtual_interface_ids: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// Peer address.
        pub peer_address: pulumi_gestalt_rust::Output<String>,
        /// Border Gateway Protocol (BGP) Autonomous System Number (ASN) of the peer.
        pub peer_bgp_asn: pulumi_gestalt_rust::Output<i32>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Virtual Local Area Network.
        pub vlan: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLocalGatewayVirtualInterfaceArgs,
    ) -> GetLocalGatewayVirtualInterfaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getLocalGatewayVirtualInterface:getLocalGatewayVirtualInterface"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLocalGatewayVirtualInterfaceResult {
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            local_address: o.get_field("localAddress"),
            local_bgp_asn: o.get_field("localBgpAsn"),
            local_gateway_id: o.get_field("localGatewayId"),
            local_gateway_virtual_interface_ids: o
                .get_field("localGatewayVirtualInterfaceIds"),
            peer_address: o.get_field("peerAddress"),
            peer_bgp_asn: o.get_field("peerBgpAsn"),
            tags: o.get_field("tags"),
            vlan: o.get_field("vlan"),
        }
    }
}

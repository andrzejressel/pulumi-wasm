#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_connect_peer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectPeerArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetConnectPeerFilter>,
            >,
        >,
        /// Key-value tags for the EC2 Transit Gateway Connect Peer
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the EC2 Transit Gateway Connect Peer.
        #[builder(into, default)]
        pub transit_gateway_connect_peer_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConnectPeerResult {
        /// EC2 Transit Gateway Connect Peer ARN
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// BGP ASN number assigned customer device
        pub bgp_asn: pulumi_gestalt_rust::Output<String>,
        /// The IP address assigned to customer device, which is used as BGP IP address.
        pub bgp_peer_address: pulumi_gestalt_rust::Output<String>,
        /// The IP addresses assigned to Transit Gateway, which are used as BGP IP addresses.
        pub bgp_transit_gateway_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetConnectPeerFilter>,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// CIDR blocks that will be used for addressing within the tunnel.
        pub inside_cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        /// IP addressed assigned to customer device, which is used as tunnel endpoint
        pub peer_address: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Connect Peer
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The IP address assigned to Transit Gateway, which is used as tunnel endpoint.
        pub transit_gateway_address: pulumi_gestalt_rust::Output<String>,
        /// The Transit Gateway Connect
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<String>,
        pub transit_gateway_connect_peer_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConnectPeerArgs,
    ) -> GetConnectPeerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_connect_peer_id_binding = args
            .transit_gateway_connect_peer_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2transitgateway/getConnectPeer:getConnectPeer".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayConnectPeerId".into(),
                    value: transit_gateway_connect_peer_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConnectPeerResult {
            arn: o.get_field("arn"),
            bgp_asn: o.get_field("bgpAsn"),
            bgp_peer_address: o.get_field("bgpPeerAddress"),
            bgp_transit_gateway_addresses: o.get_field("bgpTransitGatewayAddresses"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            inside_cidr_blocks: o.get_field("insideCidrBlocks"),
            peer_address: o.get_field("peerAddress"),
            tags: o.get_field("tags"),
            transit_gateway_address: o.get_field("transitGatewayAddress"),
            transit_gateway_attachment_id: o.get_field("transitGatewayAttachmentId"),
            transit_gateway_connect_peer_id: o.get_field("transitGatewayConnectPeerId"),
        }
    }
}

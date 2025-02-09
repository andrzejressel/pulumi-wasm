/// Resource for managing an AWS Network Manager Connect Peer.
///
/// ## Example Usage
///
/// ### Basic Usage
///
///
/// ### Usage with attachment accepter
///
///
/// ### Usage with a Tunnel-less Connect attachment
///
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_connect_peer` using the connect peer ID. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/connectPeer:ConnectPeer example connect-peer-061f3e96275db1acc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connect_peer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectPeerArgs {
        /// The Connect peer BGP options.
        #[builder(into, default)]
        pub bgp_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkmanager::ConnectPeerBgpOptions>,
        >,
        /// The ID of the connection attachment.
        #[builder(into)]
        pub connect_attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A Connect peer core network address.
        #[builder(into, default)]
        pub core_network_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The inside IP addresses used for BGP peering. Required when the Connect attachment protocol is `GRE`. See `aws.networkmanager.ConnectAttachment` for details.
        #[builder(into, default)]
        pub inside_cidr_blocks: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Connect peer address.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub peer_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The subnet ARN for the Connect peer. Required when the Connect attachment protocol is `NO_ENCAP`. See `aws.networkmanager.ConnectAttachment` for details.
        #[builder(into, default)]
        pub subnet_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectPeerResult {
        /// The ARN of the attachment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Connect peer BGP options.
        pub bgp_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkmanager::ConnectPeerBgpOptions>,
        >,
        /// The configuration of the Connect peer.
        pub configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::networkmanager::ConnectPeerConfiguration>,
        >,
        /// The ID of the connection attachment.
        pub connect_attachment_id: pulumi_gestalt_rust::Output<String>,
        pub connect_peer_id: pulumi_gestalt_rust::Output<String>,
        /// A Connect peer core network address.
        pub core_network_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of a core network.
        pub core_network_id: pulumi_gestalt_rust::Output<String>,
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The Region where the peer is located.
        pub edge_location: pulumi_gestalt_rust::Output<String>,
        /// The inside IP addresses used for BGP peering. Required when the Connect attachment protocol is `GRE`. See `aws.networkmanager.ConnectAttachment` for details.
        pub inside_cidr_blocks: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Connect peer address.
        ///
        /// The following arguments are optional:
        pub peer_address: pulumi_gestalt_rust::Output<String>,
        /// The state of the Connect peer.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The subnet ARN for the Connect peer. Required when the Connect attachment protocol is `NO_ENCAP`. See `aws.networkmanager.ConnectAttachment` for details.
        pub subnet_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectPeerArgs,
    ) -> ConnectPeerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bgp_options_binding = args.bgp_options.get_output(context);
        let connect_attachment_id_binding = args
            .connect_attachment_id
            .get_output(context);
        let core_network_address_binding = args.core_network_address.get_output(context);
        let inside_cidr_blocks_binding = args.inside_cidr_blocks.get_output(context);
        let peer_address_binding = args.peer_address.get_output(context);
        let subnet_arn_binding = args.subnet_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/connectPeer:ConnectPeer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpOptions".into(),
                    value: bgp_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectAttachmentId".into(),
                    value: connect_attachment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coreNetworkAddress".into(),
                    value: core_network_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "insideCidrBlocks".into(),
                    value: inside_cidr_blocks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerAddress".into(),
                    value: peer_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetArn".into(),
                    value: subnet_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectPeerResult {
            arn: o.get_field("arn"),
            bgp_options: o.get_field("bgpOptions"),
            configurations: o.get_field("configurations"),
            connect_attachment_id: o.get_field("connectAttachmentId"),
            connect_peer_id: o.get_field("connectPeerId"),
            core_network_address: o.get_field("coreNetworkAddress"),
            core_network_id: o.get_field("coreNetworkId"),
            created_at: o.get_field("createdAt"),
            edge_location: o.get_field("edgeLocation"),
            inside_cidr_blocks: o.get_field("insideCidrBlocks"),
            peer_address: o.get_field("peerAddress"),
            state: o.get_field("state"),
            subnet_arn: o.get_field("subnetArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}

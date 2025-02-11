/// Manages an EC2 Transit Gateway Peering Attachment.
/// For examples of custom route table association and propagation, see the [EC2 Transit Gateway Networking Examples Guide](https://docs.aws.amazon.com/vpc/latest/tgw/TGW_Scenarios.html).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   local:
///     type: aws:ec2transitgateway:TransitGateway
///     properties:
///       tags:
///         Name: Local TGW
///   peerTransitGateway:
///     type: aws:ec2transitgateway:TransitGateway
///     name: peer
///     properties:
///       tags:
///         Name: Peer TGW
///   example:
///     type: aws:ec2transitgateway:PeeringAttachment
///     properties:
///       peerAccountId: ${peerTransitGateway.ownerId}
///       peerRegion: ${peer.name}
///       peerTransitGatewayId: ${peerTransitGateway.id}
///       transitGatewayId: ${local.id}
///       tags:
///         Name: TGW Peering Requestor
/// variables:
///   peer:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_peering_attachment` using the EC2 Transit Gateway Attachment identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/peeringAttachment:PeeringAttachment example tgw-attach-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod peering_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PeeringAttachmentArgs {
        /// Describes whether dynamic routing is enabled or disabled for the transit gateway peering request. See options below for more details!
        #[builder(into, default)]
        pub options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2transitgateway::PeeringAttachmentOptions>,
        >,
        /// Account ID of EC2 Transit Gateway to peer with. Defaults to the account ID the AWS provider is currently connected to.
        #[builder(into, default)]
        pub peer_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region of EC2 Transit Gateway to peer with.
        #[builder(into)]
        pub peer_region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of EC2 Transit Gateway to peer with.
        #[builder(into)]
        pub peer_transit_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the EC2 Transit Gateway Peering Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of EC2 Transit Gateway.
        #[builder(into)]
        pub transit_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PeeringAttachmentResult {
        /// Describes whether dynamic routing is enabled or disabled for the transit gateway peering request. See options below for more details!
        pub options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2transitgateway::PeeringAttachmentOptions>,
        >,
        /// Account ID of EC2 Transit Gateway to peer with. Defaults to the account ID the AWS provider is currently connected to.
        pub peer_account_id: pulumi_gestalt_rust::Output<String>,
        /// Region of EC2 Transit Gateway to peer with.
        pub peer_region: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway to peer with.
        pub peer_transit_gateway_id: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Peering Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifier of EC2 Transit Gateway.
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PeeringAttachmentArgs,
    ) -> PeeringAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let options_binding = args.options.get_output(context);
        let peer_account_id_binding = args.peer_account_id.get_output(context);
        let peer_region_binding = args.peer_region.get_output(context);
        let peer_transit_gateway_id_binding = args
            .peer_transit_gateway_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_id_binding = args.transit_gateway_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/peeringAttachment:PeeringAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "options".into(),
                    value: &options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerAccountId".into(),
                    value: &peer_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerRegion".into(),
                    value: &peer_region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peerTransitGatewayId".into(),
                    value: &peer_transit_gateway_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PeeringAttachmentResult {
            options: o.get_field("options"),
            peer_account_id: o.get_field("peerAccountId"),
            peer_region: o.get_field("peerRegion"),
            peer_transit_gateway_id: o.get_field("peerTransitGatewayId"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            transit_gateway_id: o.get_field("transitGatewayId"),
        }
    }
}

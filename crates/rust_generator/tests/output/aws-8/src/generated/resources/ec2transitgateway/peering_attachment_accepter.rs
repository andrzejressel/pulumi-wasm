/// Manages the accepter's side of an EC2 Transit Gateway Peering Attachment.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2transitgateway:PeeringAttachmentAccepter
///     properties:
///       transitGatewayAttachmentId: ${exampleAwsEc2TransitGatewayPeeringAttachment.id}
///       tags:
///         Name: Example cross-account attachment
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_peering_attachment_accepter` using the EC2 Transit Gateway Attachment identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/peeringAttachmentAccepter:PeeringAttachmentAccepter example tgw-attach-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod peering_attachment_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PeeringAttachmentAccepterArgs {
        /// Key-value tags for the EC2 Transit Gateway Peering Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the EC2 Transit Gateway Peering Attachment to manage.
        #[builder(into)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PeeringAttachmentAccepterResult {
        /// Identifier of the AWS account that owns the EC2 TGW peering.
        pub peer_account_id: pulumi_gestalt_rust::Output<String>,
        pub peer_region: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway to peer with.
        pub peer_transit_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Peering Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the EC2 Transit Gateway Peering Attachment to manage.
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<String>,
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
        args: PeeringAttachmentAccepterArgs,
    ) -> PeeringAttachmentAccepterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/peeringAttachmentAccepter:PeeringAttachmentAccepter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PeeringAttachmentAccepterResult {
            peer_account_id: o.get_field("peerAccountId"),
            peer_region: o.get_field("peerRegion"),
            peer_transit_gateway_id: o.get_field("peerTransitGatewayId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            transit_gateway_attachment_id: o.get_field("transitGatewayAttachmentId"),
            transit_gateway_id: o.get_field("transitGatewayId"),
        }
    }
}

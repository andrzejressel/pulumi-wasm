/// Creates a transit gateway route table attachment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = transit_gateway_route_table_attachment::create(
///         "example",
///         TransitGatewayRouteTableAttachmentArgs::builder()
///             .peering_id("${exampleAwsNetworkmanagerTransitGatewayPeering.id}")
///             .transit_gateway_route_table_arn(
///                 "${exampleAwsEc2TransitGatewayRouteTable.arn}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_transit_gateway_route_table_attachment` using the attachment ID. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/transitGatewayRouteTableAttachment:TransitGatewayRouteTableAttachment example attachment-0f8fa60d2238d1bd8
/// ```
pub mod transit_gateway_route_table_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransitGatewayRouteTableAttachmentArgs {
        /// The ID of the peer for the attachment.
        #[builder(into)]
        pub peering_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the transit gateway route table for the attachment.
        #[builder(into)]
        pub transit_gateway_route_table_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TransitGatewayRouteTableAttachmentResult {
        /// Attachment Amazon Resource Name (ARN).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The policy rule number associated with the attachment.
        pub attachment_policy_rule_number: pulumi_gestalt_rust::Output<i32>,
        /// The type of attachment.
        pub attachment_type: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the core network.
        pub core_network_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the core network.
        pub core_network_id: pulumi_gestalt_rust::Output<String>,
        /// The edge location for the peer.
        pub edge_location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the attachment account owner.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the peer for the attachment.
        pub peering_id: pulumi_gestalt_rust::Output<String>,
        /// The attachment resource ARN.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the segment attachment.
        pub segment_name: pulumi_gestalt_rust::Output<String>,
        /// The state of the attachment.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ARN of the transit gateway route table for the attachment.
        pub transit_gateway_route_table_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TransitGatewayRouteTableAttachmentArgs,
    ) -> TransitGatewayRouteTableAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let peering_id_binding = args.peering_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let transit_gateway_route_table_arn_binding = args
            .transit_gateway_route_table_arn
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/transitGatewayRouteTableAttachment:TransitGatewayRouteTableAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "peeringId".into(),
                    value: &peering_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayRouteTableArn".into(),
                    value: &transit_gateway_route_table_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TransitGatewayRouteTableAttachmentResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            attachment_policy_rule_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachmentPolicyRuleNumber"),
            ),
            attachment_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachmentType"),
            ),
            core_network_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("coreNetworkArn"),
            ),
            core_network_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("coreNetworkId"),
            ),
            edge_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeLocation"),
            ),
            owner_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerAccountId"),
            ),
            peering_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("peeringId"),
            ),
            resource_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
            segment_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("segmentName"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            transit_gateway_route_table_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayRouteTableArn"),
            ),
        }
    }
}

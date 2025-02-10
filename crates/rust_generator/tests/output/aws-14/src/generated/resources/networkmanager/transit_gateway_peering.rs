/// Creates a peering connection between an AWS Cloud WAN core network and an AWS Transit Gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = transit_gateway_peering::create(
///         "example",
///         TransitGatewayPeeringArgs::builder()
///             .core_network_id("${exampleAwsccNetworkmanagerCoreNetwork.id}")
///             .transit_gateway_arn("${exampleAwsEc2TransitGateway.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_transit_gateway_peering` using the peering ID. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/transitGatewayPeering:TransitGatewayPeering example peering-444555aaabbb11223
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod transit_gateway_peering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransitGatewayPeeringArgs {
        /// The ID of a core network.
        #[builder(into)]
        pub core_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the peering. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the transit gateway for the peering request.
        #[builder(into)]
        pub transit_gateway_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TransitGatewayPeeringResult {
        /// Peering Amazon Resource Name (ARN).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the core network.
        pub core_network_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of a core network.
        pub core_network_id: pulumi_gestalt_rust::Output<String>,
        /// The edge location for the peer.
        pub edge_location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the account owner.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// The type of peering. This will be `TRANSIT_GATEWAY`.
        pub peering_type: pulumi_gestalt_rust::Output<String>,
        /// The resource ARN of the peer.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the peering. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ARN of the transit gateway for the peering request.
        pub transit_gateway_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the transit gateway peering attachment.
        pub transit_gateway_peering_attachment_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TransitGatewayPeeringArgs,
    ) -> TransitGatewayPeeringResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let core_network_id_binding = args.core_network_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_arn_binding = args.transit_gateway_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/transitGatewayPeering:TransitGatewayPeering"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coreNetworkId".into(),
                    value: core_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayArn".into(),
                    value: transit_gateway_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TransitGatewayPeeringResult {
            arn: o.get_field("arn"),
            core_network_arn: o.get_field("coreNetworkArn"),
            core_network_id: o.get_field("coreNetworkId"),
            edge_location: o.get_field("edgeLocation"),
            owner_account_id: o.get_field("ownerAccountId"),
            peering_type: o.get_field("peeringType"),
            resource_arn: o.get_field("resourceArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            transit_gateway_arn: o.get_field("transitGatewayArn"),
            transit_gateway_peering_attachment_id: o
                .get_field("transitGatewayPeeringAttachmentId"),
        }
    }
}

/// Resource for managing an AWS Network Manager Direct Connect (DX) Gateway Attachment.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = dx_gateway_attachment::create(
///         "test",
///         DxGatewayAttachmentArgs::builder()
///             .core_network_id(
///                 "${testAwsNetworkmanagerCoreNetworkPolicyAttachment.coreNetworkId}",
///             )
///             .direct_connect_gateway_arn(
///                 "arn:aws:directconnect::${current.accountId}:dx-gateway/${testAwsDxGateway.id}",
///             )
///             .edge_locations(vec!["${currentAwsRegion.name}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Manager DX Gateway Attachment using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/dxGatewayAttachment:DxGatewayAttachment example attachment-1a2b3c4d5e6f7g
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dx_gateway_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DxGatewayAttachmentArgs {
        /// ID of the Cloud WAN core network to which the Direct Connect gateway attachment should be attached.
        #[builder(into)]
        pub core_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the Direct Connect gateway attachment.
        #[builder(into)]
        pub direct_connect_gateway_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more core network edge locations to associate with the Direct Connect gateway attachment.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub edge_locations: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkmanager::DxGatewayAttachmentTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DxGatewayAttachmentResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Policy rule number associated with the attachment.
        pub attachment_policy_rule_number: pulumi_gestalt_rust::Output<i32>,
        /// Type of attachment.
        pub attachment_type: pulumi_gestalt_rust::Output<String>,
        /// ARN of the core network for the attachment.
        pub core_network_arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the Cloud WAN core network to which the Direct Connect gateway attachment should be attached.
        pub core_network_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Direct Connect gateway attachment.
        pub direct_connect_gateway_arn: pulumi_gestalt_rust::Output<String>,
        /// One or more core network edge locations to associate with the Direct Connect gateway attachment.
        ///
        /// The following arguments are optional:
        pub edge_locations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ID of the attachment account owner.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the segment attachment.
        pub segment_name: pulumi_gestalt_rust::Output<String>,
        /// State of the attachment.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkmanager::DxGatewayAttachmentTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DxGatewayAttachmentArgs,
    ) -> DxGatewayAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let core_network_id_binding = args.core_network_id.get_output(context);
        let direct_connect_gateway_arn_binding = args
            .direct_connect_gateway_arn
            .get_output(context);
        let edge_locations_binding = args.edge_locations.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/dxGatewayAttachment:DxGatewayAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coreNetworkId".into(),
                    value: core_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directConnectGatewayArn".into(),
                    value: direct_connect_gateway_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edgeLocations".into(),
                    value: edge_locations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DxGatewayAttachmentResult {
            arn: o.get_field("arn"),
            attachment_policy_rule_number: o.get_field("attachmentPolicyRuleNumber"),
            attachment_type: o.get_field("attachmentType"),
            core_network_arn: o.get_field("coreNetworkArn"),
            core_network_id: o.get_field("coreNetworkId"),
            direct_connect_gateway_arn: o.get_field("directConnectGatewayArn"),
            edge_locations: o.get_field("edgeLocations"),
            owner_account_id: o.get_field("ownerAccountId"),
            segment_name: o.get_field("segmentName"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}

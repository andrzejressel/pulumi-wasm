/// Resource for managing an AWS Network Manager SiteToSiteAttachment.
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
///     let example = site_to_site_vpn_attachment::create(
///         "example",
///         SiteToSiteVpnAttachmentArgs::builder()
///             .core_network_id("${exampleAwsccNetworkmanagerCoreNetwork.id}")
///             .vpn_connection_arn("${exampleAwsVpnConnection.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_site_to_site_vpn_attachment` using the attachment ID. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/siteToSiteVpnAttachment:SiteToSiteVpnAttachment example attachment-0f8fa60d2238d1bd8
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod site_to_site_vpn_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SiteToSiteVpnAttachmentArgs {
        /// The ID of a core network for the VPN attachment.
        #[builder(into)]
        pub core_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the site-to-site VPN connection.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vpn_connection_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SiteToSiteVpnAttachmentResult {
        /// The ARN of the attachment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The policy rule number associated with the attachment.
        pub attachment_policy_rule_number: pulumi_gestalt_rust::Output<i32>,
        /// The type of attachment.
        pub attachment_type: pulumi_gestalt_rust::Output<String>,
        /// The ARN of a core network.
        pub core_network_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of a core network for the VPN attachment.
        pub core_network_id: pulumi_gestalt_rust::Output<String>,
        /// The Region where the edge is located.
        pub edge_location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the attachment account owner.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
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
        /// The ARN of the site-to-site VPN connection.
        ///
        /// The following arguments are optional:
        pub vpn_connection_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SiteToSiteVpnAttachmentArgs,
    ) -> SiteToSiteVpnAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let core_network_id_binding = args.core_network_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpn_connection_arn_binding = args.vpn_connection_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkmanager/siteToSiteVpnAttachment:SiteToSiteVpnAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coreNetworkId".into(),
                    value: &core_network_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnConnectionArn".into(),
                    value: &vpn_connection_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SiteToSiteVpnAttachmentResult {
            arn: o.get_field("arn"),
            attachment_policy_rule_number: o.get_field("attachmentPolicyRuleNumber"),
            attachment_type: o.get_field("attachmentType"),
            core_network_arn: o.get_field("coreNetworkArn"),
            core_network_id: o.get_field("coreNetworkId"),
            edge_location: o.get_field("edgeLocation"),
            owner_account_id: o.get_field("ownerAccountId"),
            resource_arn: o.get_field("resourceArn"),
            segment_name: o.get_field("segmentName"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpn_connection_arn: o.get_field("vpnConnectionArn"),
        }
    }
}

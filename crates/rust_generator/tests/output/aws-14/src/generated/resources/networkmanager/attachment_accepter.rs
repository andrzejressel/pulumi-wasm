/// Resource for managing an AWS Network Manager Attachment Accepter.
///
/// ## Example Usage
///
/// ### Example with VPC attachment
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = attachment_accepter::create(
///         "test",
///         AttachmentAccepterArgs::builder()
///             .attachment_id("${vpc.id}")
///             .attachment_type("${vpc.attachmentType}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example with site-to-site VPN attachment
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = attachment_accepter::create(
///         "test",
///         AttachmentAccepterArgs::builder()
///             .attachment_id("${vpn.id}")
///             .attachment_type("${vpn.attachmentType}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod attachment_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AttachmentAccepterArgs {
        /// The ID of the attachment.
        #[builder(into)]
        pub attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of attachment. Valid values can be found in the [AWS Documentation](https://docs.aws.amazon.com/networkmanager/latest/APIReference/API_ListAttachments.html#API_ListAttachments_RequestSyntax)
        #[builder(into)]
        pub attachment_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AttachmentAccepterResult {
        /// The ID of the attachment.
        pub attachment_id: pulumi_gestalt_rust::Output<String>,
        /// The policy rule number associated with the attachment.
        pub attachment_policy_rule_number: pulumi_gestalt_rust::Output<i32>,
        /// The type of attachment. Valid values can be found in the [AWS Documentation](https://docs.aws.amazon.com/networkmanager/latest/APIReference/API_ListAttachments.html#API_ListAttachments_RequestSyntax)
        pub attachment_type: pulumi_gestalt_rust::Output<String>,
        /// The ARN of a core network.
        pub core_network_arn: pulumi_gestalt_rust::Output<String>,
        /// The id of a core network.
        pub core_network_id: pulumi_gestalt_rust::Output<String>,
        /// The Region where the edge is located. This is returned for all attachment types except a Direct Connect gateway attachment, which instead returns `edge_locations`.
        pub edge_location: pulumi_gestalt_rust::Output<String>,
        /// The edge locations that the Direct Connect gateway is associated with. This is returned only for Direct Connect gateway attachments. All other attachment types return `edge_location`
        pub edge_locations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the attachment account owner.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// The attachment resource ARN.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the segment attachment.
        pub segment_name: pulumi_gestalt_rust::Output<String>,
        /// The state of the attachment.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AttachmentAccepterArgs,
    ) -> AttachmentAccepterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let attachment_id_binding = args.attachment_id.get_output(context).get_inner();
        let attachment_type_binding = args
            .attachment_type
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/attachmentAccepter:AttachmentAccepter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attachmentId".into(),
                    value: &attachment_id_binding,
                },
                register_interface::ObjectField {
                    name: "attachmentType".into(),
                    value: &attachment_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AttachmentAccepterResult {
            attachment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachmentId"),
            ),
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
            edge_locations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeLocations"),
            ),
            owner_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerAccountId"),
            ),
            resource_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
            segment_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("segmentName"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
        }
    }
}

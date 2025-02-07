/// Resource for managing an AWS Network Manager ConnectAttachment.
///
/// ## Example Usage
///
/// ### Basic Usage
///
///
/// ### Usage with attachment accepter
///
///
/// ## Import
///
/// Using `pulumi import`, import `aws_networkmanager_connect_attachment` using the attachment ID. For example:
///
/// ```sh
/// $ pulumi import aws:networkmanager/connectAttachment:ConnectAttachment example attachment-0f8fa60d2238d1bd8
/// ```
pub mod connect_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectAttachmentArgs {
        /// The ID of a core network where you want to create the attachment.
        #[builder(into)]
        pub core_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Region where the edge is located.
        #[builder(into)]
        pub edge_location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Options block. See options for more information.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub options: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::networkmanager::ConnectAttachmentOptions,
        >,
        /// Key-value tags for the attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the attachment between the two connections.
        #[builder(into)]
        pub transport_attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectAttachmentResult {
        /// The ARN of the attachment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub attachment_id: pulumi_gestalt_rust::Output<String>,
        /// The policy rule number associated with the attachment.
        pub attachment_policy_rule_number: pulumi_gestalt_rust::Output<i32>,
        /// The type of attachment.
        pub attachment_type: pulumi_gestalt_rust::Output<String>,
        /// The ARN of a core network.
        pub core_network_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of a core network where you want to create the attachment.
        pub core_network_id: pulumi_gestalt_rust::Output<String>,
        /// The Region where the edge is located.
        pub edge_location: pulumi_gestalt_rust::Output<String>,
        /// Options block. See options for more information.
        ///
        /// The following arguments are optional:
        pub options: pulumi_gestalt_rust::Output<
            super::super::types::networkmanager::ConnectAttachmentOptions,
        >,
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
        /// The ID of the attachment between the two connections.
        pub transport_attachment_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ConnectAttachmentArgs,
    ) -> ConnectAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let core_network_id_binding = args
            .core_network_id
            .get_output(context)
            .get_inner();
        let edge_location_binding = args.edge_location.get_output(context).get_inner();
        let options_binding = args.options.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let transport_attachment_id_binding = args
            .transport_attachment_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkmanager/connectAttachment:ConnectAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "coreNetworkId".into(),
                    value: &core_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "edgeLocation".into(),
                    value: &edge_location_binding,
                },
                register_interface::ObjectField {
                    name: "options".into(),
                    value: &options_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transportAttachmentId".into(),
                    value: &transport_attachment_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConnectAttachmentResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
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
            options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("options"),
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
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            transport_attachment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transportAttachmentId"),
            ),
        }
    }
}

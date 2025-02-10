#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_core_network_policy_document {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCoreNetworkPolicyDocumentArgs {
        /// In a core network, all attachments use the block argument `attachment_policies` section to map an attachment to a segment. Instead of manually associating a segment to each attachment, attachments use tags, and then the tags are used to associate the attachment to the specified segment. Detailed below.
        #[builder(into, default)]
        pub attachment_policies: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentAttachmentPolicy,
                >,
            >,
        >,
        /// The core network configuration section defines the Regions where a core network should operate. For AWS Regions that are defined in the policy, the core network creates a Core Network Edge where you can connect attachments. After it's created, each Core Network Edge is peered with every other defined Region and is configured with consistent segment and routing across all Regions. Regions cannot be removed until the associated attachments are deleted. Detailed below.
        #[builder(into)]
        pub core_network_configurations: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentCoreNetworkConfiguration,
            >,
        >,
        /// Block argument that defines the service insertion actions you want to include. Detailed below.
        #[builder(into, default)]
        pub network_function_groups: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentNetworkFunctionGroup,
                >,
            >,
        >,
        /// A block argument, `segment_actions` define how routing works between segments. By default, attachments can only communicate with other attachments in the same segment. Detailed below.
        #[builder(into, default)]
        pub segment_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegmentAction,
                >,
            >,
        >,
        /// Block argument that defines the different segments in the network. Here you can provide descriptions, change defaults, and provide explicit Regional operational and route filters. The names defined for each segment are used in the `segment_actions` and `attachment_policies` section. Each segment is created, and operates, as a completely separated routing domain. By default, attachments can only communicate with other attachments in the same segment. Detailed below.
        #[builder(into)]
        pub segments: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegment,
            >,
        >,
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCoreNetworkPolicyDocumentResult {
        pub attachment_policies: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentAttachmentPolicy,
                >,
            >,
        >,
        pub core_network_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentCoreNetworkConfiguration,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Standard JSON policy document rendered based on the arguments above.
        pub json: pulumi_gestalt_rust::Output<String>,
        pub network_function_groups: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentNetworkFunctionGroup,
                >,
            >,
        >,
        pub segment_actions: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegmentAction,
                >,
            >,
        >,
        pub segments: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegment,
            >,
        >,
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCoreNetworkPolicyDocumentArgs,
    ) -> GetCoreNetworkPolicyDocumentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attachment_policies_binding = args.attachment_policies.get_output(context);
        let core_network_configurations_binding = args
            .core_network_configurations
            .get_output(context);
        let network_function_groups_binding = args
            .network_function_groups
            .get_output(context);
        let segment_actions_binding = args.segment_actions.get_output(context);
        let segments_binding = args.segments.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:networkmanager/getCoreNetworkPolicyDocument:getCoreNetworkPolicyDocument"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attachmentPolicies".into(),
                    value: attachment_policies_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coreNetworkConfigurations".into(),
                    value: core_network_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkFunctionGroups".into(),
                    value: network_function_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "segmentActions".into(),
                    value: segment_actions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "segments".into(),
                    value: segments_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCoreNetworkPolicyDocumentResult {
            attachment_policies: o.get_field("attachmentPolicies"),
            core_network_configurations: o.get_field("coreNetworkConfigurations"),
            id: o.get_field("id"),
            json: o.get_field("json"),
            network_function_groups: o.get_field("networkFunctionGroups"),
            segment_actions: o.get_field("segmentActions"),
            segments: o.get_field("segments"),
            version: o.get_field("version"),
        }
    }
}

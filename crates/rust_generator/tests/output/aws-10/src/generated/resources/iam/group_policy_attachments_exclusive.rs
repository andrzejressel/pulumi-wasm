///
///
/// ## Import
///
/// Using `pulumi import`, import exclusive management of managed IAM policy assignments using the `group_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/groupPolicyAttachmentsExclusive:GroupPolicyAttachmentsExclusive example MyGroup
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_policy_attachments_exclusive {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPolicyAttachmentsExclusiveArgs {
        /// IAM group name.
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of managed IAM policy ARNs to be attached to the group. Policies attached to this group but not configured in this argument will be removed.
        #[builder(into)]
        pub policy_arns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct GroupPolicyAttachmentsExclusiveResult {
        /// IAM group name.
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// A list of managed IAM policy ARNs to be attached to the group. Policies attached to this group but not configured in this argument will be removed.
        pub policy_arns: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupPolicyAttachmentsExclusiveArgs,
    ) -> GroupPolicyAttachmentsExclusiveResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_name_binding = args.group_name.get_output(context);
        let policy_arns_binding = args.policy_arns.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/groupPolicyAttachmentsExclusive:GroupPolicyAttachmentsExclusive"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupName".into(),
                    value: group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyArns".into(),
                    value: policy_arns_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupPolicyAttachmentsExclusiveResult {
            group_name: o.get_field("groupName"),
            policy_arns: o.get_field("policyArns"),
        }
    }
}

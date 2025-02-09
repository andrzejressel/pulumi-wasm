/// Attaches a Managed IAM Policy to an IAM group
///
/// > **NOTE:** The usage of this resource conflicts with the `aws.iam.PolicyAttachment` resource and will permanently show a difference if both are defined.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   group:
///     type: aws:iam:Group
///     properties:
///       name: test-group
///   policy:
///     type: aws:iam:Policy
///     properties:
///       name: test-policy
///       description: A test policy
///       policy: '{ ... policy JSON ... }'
///   test-attach:
///     type: aws:iam:GroupPolicyAttachment
///     properties:
///       group: ${group.name}
///       policyArn: ${policy.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM group policy attachments using the group name and policy arn separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/groupPolicyAttachment:GroupPolicyAttachment test-attach test-group/arn:aws:iam::xxxxxxxxxxxx:policy/test-policy
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_policy_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPolicyAttachmentArgs {
        /// The group the policy should be applied to
        #[builder(into)]
        pub group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the policy you want to apply
        #[builder(into)]
        pub policy_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GroupPolicyAttachmentResult {
        /// The group the policy should be applied to
        pub group: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the policy you want to apply
        pub policy_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupPolicyAttachmentArgs,
    ) -> GroupPolicyAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_binding = args.group.get_output(context);
        let policy_arn_binding = args.policy_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/groupPolicyAttachment:GroupPolicyAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "group".into(),
                    value: group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyArn".into(),
                    value: policy_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupPolicyAttachmentResult {
            group: o.get_field("group"),
            policy_arn: o.get_field("policyArn"),
        }
    }
}

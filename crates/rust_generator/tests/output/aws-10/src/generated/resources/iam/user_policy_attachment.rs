/// Attaches a Managed IAM Policy to an IAM user
///
/// > **NOTE:** The usage of this resource conflicts with the `aws.iam.PolicyAttachment` resource and will permanently show a difference if both are defined.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   user:
///     type: aws:iam:User
///     properties:
///       name: test-user
///   policy:
///     type: aws:iam:Policy
///     properties:
///       name: test-policy
///       description: A test policy
///       policy: '{ ... policy JSON ... }'
///   test-attach:
///     type: aws:iam:UserPolicyAttachment
///     properties:
///       user: ${user.name}
///       policyArn: ${policy.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM user policy attachments using the user name and policy arn separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/userPolicyAttachment:UserPolicyAttachment test-attach test-user/arn:aws:iam::xxxxxxxxxxxx:policy/test-policy
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_policy_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserPolicyAttachmentArgs {
        /// The ARN of the policy you want to apply
        #[builder(into)]
        pub policy_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user the policy should be applied to
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserPolicyAttachmentResult {
        /// The ARN of the policy you want to apply
        pub policy_arn: pulumi_gestalt_rust::Output<String>,
        /// The user the policy should be applied to
        pub user: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserPolicyAttachmentArgs,
    ) -> UserPolicyAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_arn_binding = args.policy_arn.get_output(context);
        let user_binding = args.user.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/userPolicyAttachment:UserPolicyAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyArn".into(),
                    value: policy_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: user_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserPolicyAttachmentResult {
            policy_arn: o.get_field("policyArn"),
            user: o.get_field("user"),
        }
    }
}

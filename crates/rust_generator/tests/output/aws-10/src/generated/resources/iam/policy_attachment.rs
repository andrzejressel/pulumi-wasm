/// Attaches a Managed IAM Policy to user(s), role(s), and/or group(s)
///
/// !> **WARNING:** The aws.iam.PolicyAttachment resource creates **exclusive** attachments of IAM policies. Across the entire AWS account, all of the users/roles/groups to which a single policy is attached must be declared by a single aws.iam.PolicyAttachment resource. This means that even any users/roles/groups that have the attached policy via any other mechanism (including other resources managed by this provider) will have that attached policy revoked by this resource. Consider `aws.iam.RolePolicyAttachment`, `aws.iam.UserPolicyAttachment`, or `aws.iam.GroupPolicyAttachment` instead. These resources do not enforce exclusive attachment of an IAM policy.
///
/// > **NOTE:** The usage of this resource conflicts with the `aws.iam.GroupPolicyAttachment`, `aws.iam.RolePolicyAttachment`, and `aws.iam.UserPolicyAttachment` resources and will permanently show a difference if both are defined.
///
/// > **NOTE:** For a given role, this resource is incompatible with using the `aws.iam.Role` resource `managed_policy_arns` argument. When using that argument and this resource, both will attempt to manage the role's managed policy attachments and the provider will show a permanent difference.
///
/// > **NOTE:** To ensure Pulumi correctly manages dependencies during updates, use a reference to the IAM resource when defining the `policy_arn` for `aws.iam.PolicyAttachment`, rather than constructing the ARN directly. For example, use `policy_arn = aws_iam_policy.example.arn` instead of `policy_arn = "arn:aws:iam::${data.aws_caller_identity.current.account_id}:policy/Example"`. Failing to do so may lead to errors like `DeleteConflict: Cannot delete a policy attached to entities` or `NoSuchEntity`.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   user:
///     type: aws:iam:User
///     properties:
///       name: test-user
///   role:
///     type: aws:iam:Role
///     properties:
///       name: test-role
///       assumeRolePolicy: ${assumeRole.json}
///   group:
///     type: aws:iam:Group
///     properties:
///       name: test-group
///   policyPolicy:
///     type: aws:iam:Policy
///     name: policy
///     properties:
///       name: test-policy
///       description: A test policy
///       policy: ${policy.json}
///   test-attach:
///     type: aws:iam:PolicyAttachment
///     properties:
///       name: test-attachment
///       users:
///         - ${user.name}
///       roles:
///         - ${role.name}
///       groups:
///         - ${group.name}
///       policyArn: ${policyPolicy.arn}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - ec2.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - ec2:Describe*
///             resources:
///               - '*'
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyAttachmentArgs {
        /// Group(s) the policy should be applied to.
        #[builder(into, default)]
        pub groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the attachment. This cannot be an empty string.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the policy you want to apply. Typically this should be a reference to the ARN of another resource to ensure dependency ordering, such as `aws_iam_policy.example.arn`.
        #[builder(into)]
        pub policy_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Role(s) the policy should be applied to.
        #[builder(into, default)]
        pub roles: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// User(s) the policy should be applied to.
        #[builder(into, default)]
        pub users: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct PolicyAttachmentResult {
        /// Group(s) the policy should be applied to.
        pub groups: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name of the attachment. This cannot be an empty string.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ARN of the policy you want to apply. Typically this should be a reference to the ARN of another resource to ensure dependency ordering, such as `aws_iam_policy.example.arn`.
        pub policy_arn: pulumi_gestalt_rust::Output<String>,
        /// Role(s) the policy should be applied to.
        pub roles: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// User(s) the policy should be applied to.
        pub users: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyAttachmentArgs,
    ) -> PolicyAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let groups_binding = args.groups.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_arn_binding = args.policy_arn.get_output(context);
        let roles_binding = args.roles.get_output(context);
        let users_binding = args.users.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/policyAttachment:PolicyAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groups".into(),
                    value: groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyArn".into(),
                    value: policy_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roles".into(),
                    value: roles_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "users".into(),
                    value: users_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyAttachmentResult {
            groups: o.get_field("groups"),
            name: o.get_field("name"),
            policy_arn: o.get_field("policyArn"),
            roles: o.get_field("roles"),
            users: o.get_field("users"),
        }
    }
}

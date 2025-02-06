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
pub mod policy_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyAttachmentArgs {
        /// Group(s) the policy should be applied to.
        #[builder(into, default)]
        pub groups: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the attachment. This cannot be an empty string.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ARN of the policy you want to apply. Typically this should be a reference to the ARN of another resource to ensure dependency ordering, such as `aws_iam_policy.example.arn`.
        #[builder(into)]
        pub policy_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Role(s) the policy should be applied to.
        #[builder(into, default)]
        pub roles: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// User(s) the policy should be applied to.
        #[builder(into, default)]
        pub users: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct PolicyAttachmentResult {
        /// Group(s) the policy should be applied to.
        pub groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of the attachment. This cannot be an empty string.
        pub name: pulumi_wasm_rust::Output<String>,
        /// ARN of the policy you want to apply. Typically this should be a reference to the ARN of another resource to ensure dependency ordering, such as `aws_iam_policy.example.arn`.
        pub policy_arn: pulumi_wasm_rust::Output<String>,
        /// Role(s) the policy should be applied to.
        pub roles: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// User(s) the policy should be applied to.
        pub users: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PolicyAttachmentArgs,
    ) -> PolicyAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let groups_binding = args.groups.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let policy_arn_binding = args.policy_arn.get_output(context).get_inner();
        let roles_binding = args.roles.get_output(context).get_inner();
        let users_binding = args.users.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/policyAttachment:PolicyAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groups".into(),
                    value: &groups_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policyArn".into(),
                    value: &policy_arn_binding,
                },
                register_interface::ObjectField {
                    name: "roles".into(),
                    value: &roles_binding,
                },
                register_interface::ObjectField {
                    name: "users".into(),
                    value: &users_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PolicyAttachmentResult {
            groups: pulumi_wasm_rust::__private::into_domain(o.extract_field("groups")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policy_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyArn"),
            ),
            roles: pulumi_wasm_rust::__private::into_domain(o.extract_field("roles")),
            users: pulumi_wasm_rust::__private::into_domain(o.extract_field("users")),
        }
    }
}

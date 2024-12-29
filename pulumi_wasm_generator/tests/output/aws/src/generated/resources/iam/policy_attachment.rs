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
///       Function: aws:iam:getPolicyDocument
///       Arguments:
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
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - ec2:Describe*
///             resources:
///               - '*'
/// ```
pub mod policy_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyAttachmentArgs {
        /// Group(s) the policy should be applied to.
        #[builder(into, default)]
        pub groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Name of the attachment. This cannot be an empty string.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the policy you want to apply. Typically this should be a reference to the ARN of another resource to ensure dependency ordering, such as `aws_iam_policy.example.arn`.
        #[builder(into)]
        pub policy_arn: pulumi_wasm_rust::Output<String>,
        /// Role(s) the policy should be applied to.
        #[builder(into, default)]
        pub roles: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// User(s) the policy should be applied to.
        #[builder(into, default)]
        pub users: pulumi_wasm_rust::Output<Option<Vec<String>>>,
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
    pub fn create(name: &str, args: PolicyAttachmentArgs) -> PolicyAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let groups_binding = args.groups.get_inner();
        let name_binding = args.name.get_inner();
        let policy_arn_binding = args.policy_arn.get_inner();
        let roles_binding = args.roles.get_inner();
        let users_binding = args.users.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/policyAttachment:PolicyAttachment".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "groups".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policyArn".into(),
                },
                register_interface::ResultField {
                    name: "roles".into(),
                },
                register_interface::ResultField {
                    name: "users".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyAttachmentResult {
            groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groups").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyArn").unwrap(),
            ),
            roles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roles").unwrap(),
            ),
            users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("users").unwrap(),
            ),
        }
    }
}

/// Attaches a Managed IAM Policy to an IAM role
///
/// > **NOTE:** The usage of this resource conflicts with the `aws.iam.PolicyAttachment` resource and will permanently show a difference if both are defined.
///
/// > **NOTE:** For a given role, this resource is incompatible with using the `aws.iam.Role` resource `managed_policy_arns` argument. When using that argument and this resource, both will attempt to manage the role's managed policy attachments and Pulumi will show a permanent difference.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   role:
///     type: aws:iam:Role
///     properties:
///       name: test-role
///       assumeRolePolicy: ${assumeRole.json}
///   policyPolicy:
///     type: aws:iam:Policy
///     name: policy
///     properties:
///       name: test-policy
///       description: A test policy
///       policy: ${policy.json}
///   test-attach:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       role: ${role.name}
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
///
/// ## Import
///
/// Using `pulumi import`, import IAM role policy attachments using the role name and policy arn separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/rolePolicyAttachment:RolePolicyAttachment test-attach test-role/arn:aws:iam::xxxxxxxxxxxx:policy/test-policy
/// ```
pub mod role_policy_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RolePolicyAttachmentArgs {
        /// The ARN of the policy you want to apply
        #[builder(into)]
        pub policy_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the IAM role to which the policy should be applied
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RolePolicyAttachmentResult {
        /// The ARN of the policy you want to apply
        pub policy_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the IAM role to which the policy should be applied
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RolePolicyAttachmentArgs,
    ) -> RolePolicyAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_arn_binding = args.policy_arn.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/rolePolicyAttachment:RolePolicyAttachment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyArn".into(),
                    value: &policy_arn_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "policyArn".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RolePolicyAttachmentResult {
            policy_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyArn").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
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
pub mod group_policy_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPolicyAttachmentArgs {
        /// The group the policy should be applied to
        #[builder(into)]
        pub group: pulumi_wasm_rust::Output<String>,
        /// The ARN of the policy you want to apply
        #[builder(into)]
        pub policy_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GroupPolicyAttachmentResult {
        /// The group the policy should be applied to
        pub group: pulumi_wasm_rust::Output<String>,
        /// The ARN of the policy you want to apply
        pub policy_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: GroupPolicyAttachmentArgs,
    ) -> GroupPolicyAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_binding = args.group.get_inner();
        let policy_arn_binding = args.policy_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/groupPolicyAttachment:GroupPolicyAttachment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "group".into(),
                    value: &group_binding,
                },
                register_interface::ObjectField {
                    name: "policyArn".into(),
                    value: &policy_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "group".into(),
                },
                register_interface::ResultField {
                    name: "policyArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GroupPolicyAttachmentResult {
            group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("group").unwrap(),
            ),
            policy_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyArn").unwrap(),
            ),
        }
    }
}
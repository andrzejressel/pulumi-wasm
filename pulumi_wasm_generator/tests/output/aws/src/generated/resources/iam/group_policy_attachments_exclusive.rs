///
///
/// ## Import
///
/// Using `pulumi import`, import exclusive management of managed IAM policy assignments using the `group_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/groupPolicyAttachmentsExclusive:GroupPolicyAttachmentsExclusive example MyGroup
/// ```
pub mod group_policy_attachments_exclusive {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPolicyAttachmentsExclusiveArgs {
        /// IAM group name.
        #[builder(into)]
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// A list of managed IAM policy ARNs to be attached to the group. Policies attached to this group but not configured in this argument will be removed.
        #[builder(into)]
        pub policy_arns: pulumi_wasm_rust::Output<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct GroupPolicyAttachmentsExclusiveResult {
        /// IAM group name.
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// A list of managed IAM policy ARNs to be attached to the group. Policies attached to this group but not configured in this argument will be removed.
        pub policy_arns: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: GroupPolicyAttachmentsExclusiveArgs,
    ) -> GroupPolicyAttachmentsExclusiveResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_name_binding = args.group_name.get_inner();
        let policy_arns_binding = args.policy_arns.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/groupPolicyAttachmentsExclusive:GroupPolicyAttachmentsExclusive"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
                register_interface::ObjectField {
                    name: "policyArns".into(),
                    value: &policy_arns_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "groupName".into(),
                },
                register_interface::ResultField {
                    name: "policyArns".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GroupPolicyAttachmentsExclusiveResult {
            group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupName").unwrap(),
            ),
            policy_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyArns").unwrap(),
            ),
        }
    }
}

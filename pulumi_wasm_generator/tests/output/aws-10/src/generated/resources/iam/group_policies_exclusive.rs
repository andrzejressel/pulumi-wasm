///
///
/// ## Import
///
/// Using `pulumi import`, import exclusive management of inline policy assignments using the `group_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/groupPoliciesExclusive:GroupPoliciesExclusive example MyGroup
/// ```
pub mod group_policies_exclusive {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPoliciesExclusiveArgs {
        /// IAM group name.
        #[builder(into)]
        pub group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of inline policy names to be assigned to the group. Policies attached to this group but not configured in this argument will be removed.
        #[builder(into)]
        pub policy_names: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct GroupPoliciesExclusiveResult {
        /// IAM group name.
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// A list of inline policy names to be assigned to the group. Policies attached to this group but not configured in this argument will be removed.
        pub policy_names: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GroupPoliciesExclusiveArgs,
    ) -> GroupPoliciesExclusiveResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_name_binding = args.group_name.get_output(context).get_inner();
        let policy_names_binding = args.policy_names.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/groupPoliciesExclusive:GroupPoliciesExclusive".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
                register_interface::ObjectField {
                    name: "policyNames".into(),
                    value: &policy_names_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "groupName".into(),
                },
                register_interface::ResultField {
                    name: "policyNames".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GroupPoliciesExclusiveResult {
            group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupName").unwrap(),
            ),
            policy_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyNames").unwrap(),
            ),
        }
    }
}

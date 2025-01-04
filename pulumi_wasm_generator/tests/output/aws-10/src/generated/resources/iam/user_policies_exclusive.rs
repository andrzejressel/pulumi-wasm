///
///
/// ## Import
///
/// Using `pulumi import`, import exclusive management of inline policy assignments using the `user_name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/userPoliciesExclusive:UserPoliciesExclusive example MyUser
/// ```
pub mod user_policies_exclusive {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserPoliciesExclusiveArgs {
        /// A list of inline policy names to be assigned to the user. Policies attached to this user but not configured in this argument will be removed.
        #[builder(into)]
        pub policy_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// IAM user name.
        #[builder(into)]
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserPoliciesExclusiveResult {
        /// A list of inline policy names to be assigned to the user. Policies attached to this user but not configured in this argument will be removed.
        pub policy_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// IAM user name.
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: UserPoliciesExclusiveArgs,
    ) -> UserPoliciesExclusiveResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_names_binding = args.policy_names.get_inner();
        let user_name_binding = args.user_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/userPoliciesExclusive:UserPoliciesExclusive".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyNames".into(),
                    value: &policy_names_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "policyNames".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserPoliciesExclusiveResult {
            policy_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyNames").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
        }
    }
}

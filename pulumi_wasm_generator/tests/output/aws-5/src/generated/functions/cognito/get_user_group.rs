pub mod get_user_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserGroupArgs {
        /// Name of the user group.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// User pool the client belongs to.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserGroupResult {
        /// Description of the user group.
        pub description: pulumi_wasm_rust::Output<String>,
        /// A comma-delimited string concatenating `name` and `user_pool_id`.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Precedence of the user group.
        pub precedence: pulumi_wasm_rust::Output<i32>,
        /// ARN of the IAM role to be associated with the user group.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetUserGroupArgs) -> GetUserGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let user_pool_id_binding = args.user_pool_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cognito/getUserGroup:getUserGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "precedence".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "userPoolId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetUserGroupResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            precedence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("precedence").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolId").unwrap(),
            ),
        }
    }
}

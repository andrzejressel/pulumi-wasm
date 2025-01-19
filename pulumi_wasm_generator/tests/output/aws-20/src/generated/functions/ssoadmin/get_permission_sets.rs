pub mod get_permission_sets {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPermissionSetsArgs {
        /// ARN of the SSO Instance associated with the permission set.
        #[builder(into)]
        pub instance_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetPermissionSetsResult {
        /// Set of string contain the ARN of all Permission Sets.
        pub arns: pulumi_wasm_rust::Output<Vec<String>>,
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPermissionSetsArgs) -> GetPermissionSetsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_arn_binding = args.instance_arn.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssoadmin/getPermissionSets:getPermissionSets".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arns".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceArn".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPermissionSetsResult {
            arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arns").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceArn").unwrap(),
            ),
        }
    }
}

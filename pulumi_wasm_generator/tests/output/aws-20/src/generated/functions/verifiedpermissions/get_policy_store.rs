pub mod get_policy_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyStoreArgs {
        /// The ID of the Policy Store.
        #[builder(into)]
        pub id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetPolicyStoreResult {
        /// The ARN of the Policy Store.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date the Policy Store was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// The date the Policy Store was last updated.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// Validation settings for the policy store.
        pub validation_settings: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::verifiedpermissions::GetPolicyStoreValidationSetting,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPolicyStoreArgs) -> GetPolicyStoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:verifiedpermissions/getPolicyStore:getPolicyStore".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedDate".into(),
                },
                register_interface::ResultField {
                    name: "validationSettings".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPolicyStoreResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedDate").unwrap(),
            ),
            validation_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationSettings").unwrap(),
            ),
        }
    }
}

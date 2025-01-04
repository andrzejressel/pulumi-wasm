pub mod get_lifecycle_policy_document {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLifecyclePolicyDocumentArgs {
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ecr::GetLifecyclePolicyDocumentRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLifecyclePolicyDocumentResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The above arguments serialized as a standard JSON policy document.
        pub json: pulumi_wasm_rust::Output<String>,
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ecr::GetLifecyclePolicyDocumentRule>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetLifecyclePolicyDocumentArgs,
    ) -> GetLifecyclePolicyDocumentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let rules_binding = args.rules.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecr/getLifecyclePolicyDocument:getLifecyclePolicyDocument"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "json".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLifecyclePolicyDocumentResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("json").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
        }
    }
}

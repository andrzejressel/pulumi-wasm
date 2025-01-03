pub mod get_functions {
    #[allow(dead_code)]
    pub struct GetFunctionsResult {
        /// A list of Lambda Function ARNs.
        pub function_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of Lambda Function names.
        pub function_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke() -> GetFunctionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lambda/getFunctions:getFunctions".into(),
            object: Vec::from([]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "functionArns".into(),
                },
                register_interface::ResultField {
                    name: "functionNames".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFunctionsResult {
            function_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionArns").unwrap(),
            ),
            function_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionNames").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}

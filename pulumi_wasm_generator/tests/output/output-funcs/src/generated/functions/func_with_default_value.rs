pub mod func_with_default_value {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithDefaultValueArgs {
        #[builder(into)]
        pub a: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub b: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FuncWithDefaultValueResult {
        pub r: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: FuncWithDefaultValueArgs) -> FuncWithDefaultValueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let a_binding = args.a.get_inner();
        let b_binding = args.b.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::funcWithDefaultValue".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "a".into(),
                    value: &a_binding,
                },
                register_interface::ObjectField {
                    name: "b".into(),
                    value: &b_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "r".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FuncWithDefaultValueResult {
            r: pulumi_wasm_rust::__private::into_domain(hashmap.remove("r").unwrap()),
        }
    }
}

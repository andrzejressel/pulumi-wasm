pub mod func_with_all_optional_inputs {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithAllOptionalInputsArgs {
        /// Property A
        #[builder(into, default)]
        pub a: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::HelmReleaseSettings>,
        >,
        /// Property B
        #[builder(into, default)]
        pub b: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FuncWithAllOptionalInputsResult {
        pub r: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: FuncWithAllOptionalInputsArgs,
    ) -> FuncWithAllOptionalInputsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let a_binding = args.a.get_output(context).get_inner();
        let b_binding = args.b.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::funcWithAllOptionalInputs".into(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FuncWithAllOptionalInputsResult {
            r: pulumi_wasm_rust::__private::into_domain(hashmap.remove("r").unwrap()),
        }
    }
}

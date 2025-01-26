pub mod func_with_dict_param {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithDictParamArgs {
        #[builder(into, default)]
        pub a: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub b: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FuncWithDictParamResult {
        pub r: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: FuncWithDictParamArgs,
    ) -> FuncWithDictParamResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let a_binding = args.a.get_output(context).get_inner();
        let b_binding = args.b.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::funcWithDictParam".into(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        FuncWithDictParamResult {
            r: pulumi_wasm_rust::__private::into_domain(o.extract_field("r")),
        }
    }
}

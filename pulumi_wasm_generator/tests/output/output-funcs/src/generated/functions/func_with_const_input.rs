pub mod func_with_const_input {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FuncWithConstInputArgs {
        #[builder(into, default)]
        pub plain_input: pulumi_wasm_rust::Output<
            Option<super::super::constants::ConstStringFixed>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: FuncWithConstInputArgs) {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let plain_input_binding = args.plain_input.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::funcWithConstInput".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "plainInput".into(),
                    value: &plain_input_binding,
                },
            ]),
            results: Vec::from([]),
        };
        register_interface::invoke(&request);
    }
}

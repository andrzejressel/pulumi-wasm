pub mod type_ {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TypeArgs {
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<
            Option<super::super::super::super::super::types::impl_::let_::loop_::Type>,
        >,
    }
    #[allow(dead_code)]
    pub struct TypeResult {
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: TypeArgs) -> TypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let type__binding = args.type_.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "example:impl/let/loop:Type".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TypeResult {
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}

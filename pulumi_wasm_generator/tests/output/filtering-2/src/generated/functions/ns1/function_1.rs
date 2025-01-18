pub mod function_1 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Function1Args {
        #[builder(into, default)]
        pub common_type: pulumi_wasm_rust::Output<
            Option<super::super::super::types::common::CommonType>,
        >,
        #[builder(into, default)]
        pub type1: pulumi_wasm_rust::Output<
            Option<super::super::super::types::ns1::Type1>,
        >,
    }
    #[allow(dead_code)]
    pub struct Function1Result {
        pub result: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: Function1Args) -> Function1Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let common_type_binding = args.common_type.get_inner();
        let type1_binding = args.type1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "example:ns1:Function1".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "commonType".into(),
                    value: &common_type_binding,
                },
                register_interface::ObjectField {
                    name: "type1".into(),
                    value: &type1_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "result".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        Function1Result {
            result: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("result").unwrap(),
            ),
        }
    }
}

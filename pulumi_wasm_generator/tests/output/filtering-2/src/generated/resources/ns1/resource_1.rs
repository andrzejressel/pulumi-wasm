pub mod resource_1 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Resource1Args {
        #[builder(into, default)]
        pub common_type: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::common::CommonType>,
        >,
        #[builder(into, default)]
        pub type1: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ns1::Type1>,
        >,
    }
    #[allow(dead_code)]
    pub struct Resource1Result {
        pub common_type: pulumi_wasm_rust::Output<
            Option<super::super::types::common::CommonType>,
        >,
        pub type1: pulumi_wasm_rust::Output<Option<super::super::types::ns1::Type1>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: Resource1Args,
    ) -> Resource1Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let common_type_binding = args.common_type.get_output(context).get_inner();
        let type1_binding = args.type1.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "example:ns1:Resource1".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "commonType".into(),
                },
                register_interface::ResultField {
                    name: "type1".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        Resource1Result {
            common_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("commonType").unwrap(),
            ),
            type1: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type1").unwrap(),
            ),
        }
    }
}

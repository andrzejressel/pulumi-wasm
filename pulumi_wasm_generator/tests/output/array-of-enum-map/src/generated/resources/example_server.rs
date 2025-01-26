pub mod example_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExampleServerArgs {
        #[builder(into, default)]
        pub map_array_enum: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    std::collections::HashMap<
                        String,
                        super::types::AnnotationStoreSchemaValueType,
                    >,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ExampleServerResult {
        pub map_array_enum: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    std::collections::HashMap<
                        String,
                        super::types::AnnotationStoreSchemaValueType,
                    >,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ExampleServerArgs,
    ) -> ExampleServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let map_array_enum_binding = args.map_array_enum.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "example:index:ExampleServer".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mapArrayEnum".into(),
                    value: &map_array_enum_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "mapArrayEnum".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExampleServerResult {
            map_array_enum: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mapArrayEnum").unwrap(),
            ),
        }
    }
}

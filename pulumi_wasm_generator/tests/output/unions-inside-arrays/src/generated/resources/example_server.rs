pub mod example_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExampleServerArgs {
        #[builder(into, default)]
        pub properties_collection: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    pulumi_wasm_rust::OneOf2<
                        super::types::ServerPropertiesForReplica,
                        super::types::ServerPropertiesForRestore,
                    >,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ExampleServerResult {
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ExampleServerArgs) -> ExampleServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let properties_collection_binding = args.properties_collection.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "example:index:ExampleServer".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "propertiesCollection".into(),
                    value: &properties_collection_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExampleServerResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}

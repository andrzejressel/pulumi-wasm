pub mod resource {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        #[builder(into, default)]
        pub baz: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        pub baz: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResourceArgs) -> ResourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let baz_binding = args.baz.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "foo-bar:deeply/nested/module:Resource".into(),
            name: name.to_string(),
            version: super::super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "baz".into(),
                    value: &baz_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "baz".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceResult {
            baz: pulumi_wasm_rust::__private::into_domain(hashmap.remove("baz").unwrap()),
        }
    }
}

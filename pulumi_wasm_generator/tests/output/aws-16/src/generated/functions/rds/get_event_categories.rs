pub mod get_event_categories {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEventCategoriesArgs {
        /// Type of source that will be generating the events. Valid options are db-instance, db-security-group, db-parameter-group, db-snapshot, db-cluster or db-cluster-snapshot.
        #[builder(into, default)]
        pub source_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEventCategoriesResult {
        /// List of the event categories.
        pub event_categories: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub source_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetEventCategoriesArgs) -> GetEventCategoriesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let source_type_binding = args.source_type.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getEventCategories:getEventCategories".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "sourceType".into(),
                    value: &source_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "eventCategories".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "sourceType".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEventCategoriesResult {
            event_categories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventCategories").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            source_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceType").unwrap(),
            ),
        }
    }
}

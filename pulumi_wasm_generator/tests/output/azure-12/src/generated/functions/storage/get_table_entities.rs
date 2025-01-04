pub mod get_table_entities {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableEntitiesArgs {
        /// The filter used to retrieve the entities.
        #[builder(into)]
        pub filter: pulumi_wasm_rust::Output<String>,
        /// A list of properties to select from the returned Storage Table Entities.
        #[builder(into, default)]
        pub selects: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Storage Table ID where the entities exist.
        #[builder(into)]
        pub storage_table_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetTableEntitiesResult {
        pub filter: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of `items` blocks as defined below.
        pub items: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::storage::GetTableEntitiesItem>,
        >,
        pub selects: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub storage_table_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTableEntitiesArgs) -> GetTableEntitiesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filter_binding = args.filter.get_inner();
        let selects_binding = args.selects.get_inner();
        let storage_table_id_binding = args.storage_table_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getTableEntities:getTableEntities".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "selects".into(),
                    value: &selects_binding,
                },
                register_interface::ObjectField {
                    name: "storageTableId".into(),
                    value: &storage_table_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "items".into(),
                },
                register_interface::ResultField {
                    name: "selects".into(),
                },
                register_interface::ResultField {
                    name: "storageTableId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTableEntitiesResult {
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            items: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("items").unwrap(),
            ),
            selects: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selects").unwrap(),
            ),
            storage_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageTableId").unwrap(),
            ),
        }
    }
}

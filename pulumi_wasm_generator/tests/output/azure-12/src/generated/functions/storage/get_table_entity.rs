pub mod get_table_entity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableEntityArgs {
        /// The key for the partition where the entity will be retrieved.
        #[builder(into)]
        pub partition_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// The key for the row where the entity will be retrieved.
        #[builder(into)]
        pub row_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Storage Table ID where the entity exists.
        #[builder(into)]
        pub storage_table_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTableEntityResult {
        /// A map of key/value pairs that describe the entity to be stored in the storage table.
        pub entity: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub partition_key: pulumi_wasm_rust::Output<String>,
        pub row_key: pulumi_wasm_rust::Output<String>,
        pub storage_table_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTableEntityArgs,
    ) -> GetTableEntityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let partition_key_binding = args.partition_key.get_output(context).get_inner();
        let row_key_binding = args.row_key.get_output(context).get_inner();
        let storage_table_id_binding = args
            .storage_table_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getTableEntity:getTableEntity".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "partitionKey".into(),
                    value: &partition_key_binding,
                },
                register_interface::ObjectField {
                    name: "rowKey".into(),
                    value: &row_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageTableId".into(),
                    value: &storage_table_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "entity".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "partitionKey".into(),
                },
                register_interface::ResultField {
                    name: "rowKey".into(),
                },
                register_interface::ResultField {
                    name: "storageTableId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTableEntityResult {
            entity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entity").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            partition_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionKey").unwrap(),
            ),
            row_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rowKey").unwrap(),
            ),
            storage_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageTableId").unwrap(),
            ),
        }
    }
}

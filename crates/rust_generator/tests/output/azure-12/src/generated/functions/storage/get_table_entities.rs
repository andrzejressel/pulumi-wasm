pub mod get_table_entities {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableEntitiesArgs {
        /// The filter used to retrieve the entities.
        #[builder(into)]
        pub filter: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of properties to select from the returned Storage Table Entities.
        #[builder(into, default)]
        pub selects: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Storage Table ID where the entities exist.
        #[builder(into)]
        pub storage_table_id: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTableEntitiesArgs,
    ) -> GetTableEntitiesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filter_binding = args.filter.get_output(context).get_inner();
        let selects_binding = args.selects.get_output(context).get_inner();
        let storage_table_id_binding = args
            .storage_table_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getTableEntities:getTableEntities".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTableEntitiesResult {
            filter: pulumi_wasm_rust::__private::into_domain(o.extract_field("filter")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            items: pulumi_wasm_rust::__private::into_domain(o.extract_field("items")),
            selects: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selects"),
            ),
            storage_table_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageTableId"),
            ),
        }
    }
}

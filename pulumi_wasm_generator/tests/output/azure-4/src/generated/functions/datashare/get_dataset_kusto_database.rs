pub mod get_dataset_kusto_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatasetKustoDatabaseArgs {
        /// The name of this Data Share Kusto Database Dataset.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource ID of the Data Share where this Data Share Kusto Database Dataset should be created.
        #[builder(into)]
        pub share_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDatasetKustoDatabaseResult {
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The location of the Kusto Cluster.
        pub kusto_cluster_location: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Kusto Cluster Database to be shared with the receiver.
        pub kusto_database_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub share_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDatasetKustoDatabaseArgs,
    ) -> GetDatasetKustoDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let share_id_binding = args.share_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:datashare/getDatasetKustoDatabase:getDatasetKustoDatabase"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "shareId".into(),
                    value: &share_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDatasetKustoDatabaseResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kusto_cluster_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kustoClusterLocation"),
            ),
            kusto_database_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kustoDatabaseId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            share_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shareId"),
            ),
        }
    }
}

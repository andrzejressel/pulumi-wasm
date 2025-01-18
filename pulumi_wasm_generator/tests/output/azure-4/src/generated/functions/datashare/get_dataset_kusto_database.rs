pub mod get_dataset_kusto_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatasetKustoDatabaseArgs {
        /// The name of this Data Share Kusto Database Dataset.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Data Share where this Data Share Kusto Database Dataset should be created.
        #[builder(into)]
        pub share_id: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetDatasetKustoDatabaseArgs) -> GetDatasetKustoDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let share_id_binding = args.share_id.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kustoClusterLocation".into(),
                },
                register_interface::ResultField {
                    name: "kustoDatabaseId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "shareId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDatasetKustoDatabaseResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kusto_cluster_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kustoClusterLocation").unwrap(),
            ),
            kusto_database_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kustoDatabaseId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            share_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareId").unwrap(),
            ),
        }
    }
}

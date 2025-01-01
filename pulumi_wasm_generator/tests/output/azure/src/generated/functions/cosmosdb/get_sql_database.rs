pub mod get_sql_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSqlDatabaseArgs {
        /// The name of the Cosmos DB SQL Database to create the table within.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Cosmos DB SQL Database.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Cosmos DB SQL Database is created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSqlDatabaseResult {
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// An `autoscale_settings` block as defined below.
        pub autoscale_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetSqlDatabaseAutoscaleSetting>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The throughput of SQL database (RU/s).
        pub throughput: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSqlDatabaseArgs) -> GetSqlDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cosmosdb/getSqlDatabase:getSqlDatabase".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "autoscaleSettings".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "throughput".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSqlDatabaseResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            autoscale_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscaleSettings").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throughput").unwrap(),
            ),
        }
    }
}

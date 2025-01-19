/// An Azure Cosmos DB container.
/// API Version: 2021-03-15.
///
/// ## Example Usage
/// ### CosmosDBSqlContainerCreateUpdate
///
///
///
///
///
/// ## Import
///
/// An existing resource can be imported using its type token, name, and identifier, e.g.
///
/// ```sh
/// $ pulumi import azure-native:documentdb:SqlResourceSqlContainer containerName /subscriptions/subid/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/ddb1/sqlDatabases/databaseName/sqlContainers/containerName
/// ```
pub mod sql_resource_sql_container {
    #[allow(dead_code)]
    pub struct SqlResourceSqlContainerResult {
        pub resource: pulumi_wasm_rust::Output<
            Option<
                super::super::types::documentdb::SqlContainerGetPropertiesResponseResource,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str) -> SqlResourceSqlContainerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::RegisterResourceRequest {
            type_: "azure-native:documentdb:SqlResourceSqlContainer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "resource".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SqlResourceSqlContainerResult {
            resource: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resource").unwrap(),
            ),
        }
    }
}

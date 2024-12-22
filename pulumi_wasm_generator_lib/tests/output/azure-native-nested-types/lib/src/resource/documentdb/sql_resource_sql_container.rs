//! An Azure Cosmos DB container.
//! API Version: 2021-03-15.
//! 
//! ## Example Usage
//! ### CosmosDBSqlContainerCreateUpdate
//! 
//! 
//! 
//! 
//! 
//! ## Import
//! 
//! An existing resource can be imported using its type token, name, and identifier, e.g.
//! 
//! ```sh
//! $ pulumi import azure-native:documentdb:SqlResourceSqlContainer containerName /subscriptions/subid/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/ddb1/sqlDatabases/databaseName/sqlContainers/containerName 
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SqlResourceSqlContainerArgs {
}

pub struct SqlResourceSqlContainerResult {
    pub resource: pulumi_wasm_rust::Output<Option<crate::types::documentdb::SqlContainerGetPropertiesResponseResource>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
) -> SqlResourceSqlContainerResult {

    let result = crate::bindings::pulumi::azure_native::documentdb_sql_resource_sql_container::invoke(
        name,
    );

    SqlResourceSqlContainerResult {
        resource: crate::into_domain(result.resource),
    }
}

/// Manages a SQL Stored Procedure within a Cosmos DB Account SQL Database.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleSqlDatabase:
///     type: azure:cosmosdb:SqlDatabase
///     name: example
///     properties:
///       name: tfex-cosmos-db
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///       throughput: 400
///   exampleSqlContainer:
///     type: azure:cosmosdb:SqlContainer
///     name: example
///     properties:
///       name: example-container
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///       databaseName: ${exampleSqlDatabase.name}
///       partitionKeyPath: /id
///   exampleSqlStoredProcedure:
///     type: azure:cosmosdb:SqlStoredProcedure
///     name: example
///     properties:
///       name: test-stored-proc
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///       databaseName: ${exampleSqlDatabase.name}
///       containerName: ${exampleSqlContainer.name}
///       body: |2
///            function () { var context = getContext(); var response = context.getResponse(); response.setBody('Hello, World'); }
/// variables:
///   example:
///     fn::invoke:
///       function: azure:cosmosdb:getAccount
///       arguments:
///         name: tfex-cosmosdb-account
///         resourceGroupName: tfex-cosmosdb-account-rg
/// ```
///
/// ## Import
///
/// CosmosDB SQL Stored Procedures can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/sqlStoredProcedure:SqlStoredProcedure db1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/account1/sqlDatabases/db1/containers/c1/storedProcedures/sp1
/// ```
///
pub mod sql_stored_procedure {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlStoredProcedureArgs {
        /// The name of the Cosmos DB Account to create the stored procedure within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The body of the stored procedure.
        #[builder(into)]
        pub body: pulumi_wasm_rust::Output<String>,
        /// The name of the Cosmos DB SQL Container to create the stored procedure within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Cosmos DB SQL Database to create the stored procedure within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Cosmos DB SQL Stored Procedure. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the Cosmos DB SQL Database is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SqlStoredProcedureResult {
        /// The name of the Cosmos DB Account to create the stored procedure within. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The body of the stored procedure.
        pub body: pulumi_wasm_rust::Output<String>,
        /// The name of the Cosmos DB SQL Container to create the stored procedure within. Changing this forces a new resource to be created.
        pub container_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Cosmos DB SQL Database to create the stored procedure within. Changing this forces a new resource to be created.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Cosmos DB SQL Stored Procedure. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Cosmos DB SQL Database is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SqlStoredProcedureArgs) -> SqlStoredProcedureResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let body_binding = args.body.get_inner();
        let container_name_binding = args.container_name.get_inner();
        let database_name_binding = args.database_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/sqlStoredProcedure:SqlStoredProcedure".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "body".into(),
                    value: &body_binding,
                },
                register_interface::ObjectField {
                    name: "containerName".into(),
                    value: &container_name_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
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
                    name: "body".into(),
                },
                register_interface::ResultField {
                    name: "containerName".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SqlStoredProcedureResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("body").unwrap(),
            ),
            container_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerName").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}

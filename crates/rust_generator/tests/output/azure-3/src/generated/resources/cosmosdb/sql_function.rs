/// Manages an SQL User Defined Function.
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
///   exampleSqlFunction:
///     type: azure:cosmosdb:SqlFunction
///     name: example
///     properties:
///       name: test-function
///       containerId: ${exampleSqlContainer.id}
///       body: function trigger(){}
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
/// SQL User Defined Functions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/sqlFunction:SqlFunction example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DocumentDB/databaseAccounts/account1/sqlDatabases/database1/containers/container1/userDefinedFunctions/userDefinedFunction1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sql_function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlFunctionArgs {
        /// Body of the User Defined Function.
        #[builder(into)]
        pub body: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the Cosmos DB SQL Container to create the SQL User Defined Function within. Changing this forces a new SQL User Defined Function to be created.
        #[builder(into)]
        pub container_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this SQL User Defined Function. Changing this forces a new SQL User Defined Function to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SqlFunctionResult {
        /// Body of the User Defined Function.
        pub body: pulumi_gestalt_rust::Output<String>,
        /// The id of the Cosmos DB SQL Container to create the SQL User Defined Function within. Changing this forces a new SQL User Defined Function to be created.
        pub container_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this SQL User Defined Function. Changing this forces a new SQL User Defined Function to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SqlFunctionArgs,
    ) -> SqlFunctionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let body_binding = args.body.get_output(context);
        let container_id_binding = args.container_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/sqlFunction:SqlFunction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "body".into(),
                    value: body_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerId".into(),
                    value: container_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SqlFunctionResult {
            body: o.get_field("body"),
            container_id: o.get_field("containerId"),
            name: o.get_field("name"),
        }
    }
}

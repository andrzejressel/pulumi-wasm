/// Manages an SQL Trigger.
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
///   exampleSqlTrigger:
///     type: azure:cosmosdb:SqlTrigger
///     name: example
///     properties:
///       name: test-trigger
///       containerId: ${exampleSqlContainer.id}
///       body: function trigger(){}
///       operation: Delete
///       type: Post
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
/// SQL Triggers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/sqlTrigger:SqlTrigger example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DocumentDB/databaseAccounts/account1/sqlDatabases/database1/containers/container1/triggers/trigger1
/// ```
///
pub mod sql_trigger {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlTriggerArgs {
        /// Body of the Trigger.
        #[builder(into)]
        pub body: pulumi_wasm_rust::InputOrOutput<String>,
        /// The id of the Cosmos DB SQL Container to create the SQL Trigger within. Changing this forces a new SQL Trigger to be created.
        #[builder(into)]
        pub container_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this SQL Trigger. Changing this forces a new SQL Trigger to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The operation the trigger is associated with. Possible values are `All`, `Create`, `Update`, `Delete` and `Replace`.
        #[builder(into)]
        pub operation: pulumi_wasm_rust::InputOrOutput<String>,
        /// Type of the Trigger. Possible values are `Pre` and `Post`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SqlTriggerResult {
        /// Body of the Trigger.
        pub body: pulumi_wasm_rust::Output<String>,
        /// The id of the Cosmos DB SQL Container to create the SQL Trigger within. Changing this forces a new SQL Trigger to be created.
        pub container_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this SQL Trigger. Changing this forces a new SQL Trigger to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The operation the trigger is associated with. Possible values are `All`, `Create`, `Update`, `Delete` and `Replace`.
        pub operation: pulumi_wasm_rust::Output<String>,
        /// Type of the Trigger. Possible values are `Pre` and `Post`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SqlTriggerArgs,
    ) -> SqlTriggerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let body_binding = args.body.get_output(context).get_inner();
        let container_id_binding = args.container_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let operation_binding = args.operation.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/sqlTrigger:SqlTrigger".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "body".into(),
                    value: &body_binding,
                },
                register_interface::ObjectField {
                    name: "containerId".into(),
                    value: &container_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "operation".into(),
                    value: &operation_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SqlTriggerResult {
            body: pulumi_wasm_rust::__private::into_domain(o.extract_field("body")),
            container_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            operation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("operation"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}

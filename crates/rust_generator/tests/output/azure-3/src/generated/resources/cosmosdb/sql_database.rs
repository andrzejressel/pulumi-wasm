/// Manages a SQL Database within a Cosmos DB Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleSqlDatabase:
///     type: azure:cosmosdb:SqlDatabase
///     name: example
///     properties:
///       name: tfex-cosmos-sql-db
///       resourceGroupName: ${example.resourceGroupName}
///       accountName: ${example.name}
///       throughput: 400
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
/// Cosmos SQL Database can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/sqlDatabase:SqlDatabase db1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/account1/sqlDatabases/db1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sql_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlDatabaseArgs {
        /// The name of the Cosmos DB SQL Database to create the table within. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        #[builder(into, default)]
        pub autoscale_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cosmosdb::SqlDatabaseAutoscaleSettings>,
        >,
        /// Specifies the name of the Cosmos DB SQL Database. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Cosmos DB SQL Database is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub throughput: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct SqlDatabaseResult {
        /// The name of the Cosmos DB SQL Database to create the table within. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// An `autoscale_settings` block as defined below. This must be set upon database creation otherwise it cannot be updated without a manual destroy-apply.
        ///
        /// > **Note:** Switching between autoscale and manual throughput is not supported via this provider and must be completed via the Azure Portal and refreshed.
        pub autoscale_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::cosmosdb::SqlDatabaseAutoscaleSettings>,
        >,
        /// Specifies the name of the Cosmos DB SQL Database. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Cosmos DB SQL Database is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub throughput: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SqlDatabaseArgs,
    ) -> SqlDatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_output(context).get_inner();
        let autoscale_settings_binding = args
            .autoscale_settings
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let throughput_binding = args.throughput.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/sqlDatabase:SqlDatabase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "autoscaleSettings".into(),
                    value: &autoscale_settings_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "throughput".into(),
                    value: &throughput_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SqlDatabaseResult {
            account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            autoscale_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoscaleSettings"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            throughput: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("throughput"),
            ),
        }
    }
}

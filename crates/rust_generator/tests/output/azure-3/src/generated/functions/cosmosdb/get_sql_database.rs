#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_sql_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSqlDatabaseArgs {
        /// The name of the Cosmos DB SQL Database to create the table within.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Cosmos DB SQL Database.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the Cosmos DB SQL Database is created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSqlDatabaseResult {
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// An `autoscale_settings` block as defined below.
        pub autoscale_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetSqlDatabaseAutoscaleSetting>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The throughput of SQL database (RU/s).
        pub throughput: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSqlDatabaseArgs,
    ) -> GetSqlDatabaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:cosmosdb/getSqlDatabase:getSqlDatabase".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSqlDatabaseResult {
            account_name: o.get_field("accountName"),
            autoscale_settings: o.get_field("autoscaleSettings"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            throughput: o.get_field("throughput"),
        }
    }
}

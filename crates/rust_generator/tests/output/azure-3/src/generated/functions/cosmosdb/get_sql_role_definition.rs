#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_sql_role_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSqlRoleDefinitionArgs {
        /// The name of the Cosmos DB Account.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Cosmos DB SQL Role Definition is created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The GUID as the name of the Cosmos DB SQL Role Definition.
        #[builder(into)]
        pub role_definition_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSqlRoleDefinitionResult {
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// A list of fully qualified scopes at or below which Role Assignments may be created using this Cosmos DB SQL Role Definition.
        pub assignable_scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The role name of the Cosmos DB SQL Role Definition.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `permissions` block as defined below.
        pub permissions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetSqlRoleDefinitionPermission>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub role_definition_id: pulumi_gestalt_rust::Output<String>,
        /// The type of the Cosmos DB SQL Role Definition.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSqlRoleDefinitionArgs,
    ) -> GetSqlRoleDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let role_definition_id_binding = args.role_definition_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:cosmosdb/getSqlRoleDefinition:getSqlRoleDefinition".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleDefinitionId".into(),
                    value: &role_definition_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSqlRoleDefinitionResult {
            account_name: o.get_field("accountName"),
            assignable_scopes: o.get_field("assignableScopes"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            permissions: o.get_field("permissions"),
            resource_group_name: o.get_field("resourceGroupName"),
            role_definition_id: o.get_field("roleDefinitionId"),
            type_: o.get_field("type"),
        }
    }
}

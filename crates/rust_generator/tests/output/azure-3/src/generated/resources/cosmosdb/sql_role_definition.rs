/// Manages a Cosmos DB SQL Role Definition.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:cosmosdb:Account
///     name: example
///     properties:
///       name: example-cosmosdb
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       offerType: Standard
///       kind: GlobalDocumentDB
///       consistencyPolicy:
///         consistencyLevel: Strong
///       geoLocations:
///         - location: ${example.location}
///           failoverPriority: 0
///   exampleSqlRoleDefinition:
///     type: azure:cosmosdb:SqlRoleDefinition
///     name: example
///     properties:
///       roleDefinitionId: 84cf3a8b-4122-4448-bce2-fa423cfe0a15
///       resourceGroupName: ${example.name}
///       accountName: ${exampleAccount.name}
///       name: acctestsqlrole
///       assignableScopes:
///         - ${exampleAccount.id}/dbs/sales
///       permissions:
///         - dataActions:
///             - Microsoft.DocumentDB/databaseAccounts/sqlDatabases/containers/items/read
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Cosmos DB SQL Role Definitions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/sqlRoleDefinition:SqlRoleDefinition example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DocumentDB/databaseAccounts/account1/sqlRoleDefinitions/28b3c337-f436-482b-a167-c2618dc52033
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sql_role_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlRoleDefinitionArgs {
        /// The name of the Cosmos DB Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of fully qualified scopes at or below which Role Assignments may be created using this Cosmos DB SQL Role Definition. It will allow application of this Cosmos DB SQL Role Definition on the entire Database Account or any underlying Database/Collection. Scopes higher than Database Account are not enforceable as assignable scopes.
        ///
        /// > **NOTE:** The resources referenced in assignable scopes need not exist.
        #[builder(into)]
        pub assignable_scopes: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// An user-friendly name for the Cosmos DB SQL Role Definition which must be unique for the Database Account.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `permissions` block as defined below.
        #[builder(into)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::cosmosdb::SqlRoleDefinitionPermission>,
        >,
        /// The name of the Resource Group in which the Cosmos DB SQL Role Definition is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The GUID as the name of the Cosmos DB SQL Role Definition - one will be generated if not specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub role_definition_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the Cosmos DB SQL Role Definition. Possible values are `BuiltInRole` and `CustomRole`. Defaults to `CustomRole`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SqlRoleDefinitionResult {
        /// The name of the Cosmos DB Account. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// A list of fully qualified scopes at or below which Role Assignments may be created using this Cosmos DB SQL Role Definition. It will allow application of this Cosmos DB SQL Role Definition on the entire Database Account or any underlying Database/Collection. Scopes higher than Database Account are not enforceable as assignable scopes.
        ///
        /// > **NOTE:** The resources referenced in assignable scopes need not exist.
        pub assignable_scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// An user-friendly name for the Cosmos DB SQL Role Definition which must be unique for the Database Account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `permissions` block as defined below.
        pub permissions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cosmosdb::SqlRoleDefinitionPermission>,
        >,
        /// The name of the Resource Group in which the Cosmos DB SQL Role Definition is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The GUID as the name of the Cosmos DB SQL Role Definition - one will be generated if not specified. Changing this forces a new resource to be created.
        pub role_definition_id: pulumi_gestalt_rust::Output<String>,
        /// The type of the Cosmos DB SQL Role Definition. Possible values are `BuiltInRole` and `CustomRole`. Defaults to `CustomRole`. Changing this forces a new resource to be created.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SqlRoleDefinitionArgs,
    ) -> SqlRoleDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let assignable_scopes_binding = args.assignable_scopes.get_output(context);
        let name_binding = args.name.get_output(context);
        let permissions_binding = args.permissions.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let role_definition_id_binding = args.role_definition_id.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/sqlRoleDefinition:SqlRoleDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assignableScopes".into(),
                    value: &assignable_scopes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleDefinitionId".into(),
                    value: &role_definition_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SqlRoleDefinitionResult {
            account_name: o.get_field("accountName"),
            assignable_scopes: o.get_field("assignableScopes"),
            name: o.get_field("name"),
            permissions: o.get_field("permissions"),
            resource_group_name: o.get_field("resourceGroupName"),
            role_definition_id: o.get_field("roleDefinitionId"),
            type_: o.get_field("type"),
        }
    }
}

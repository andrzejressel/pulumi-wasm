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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SqlRoleDefinitionArgs,
    ) -> SqlRoleDefinitionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_name_binding_1 = args.account_name.get_output(context);
        let account_name_binding = account_name_binding_1.get_inner();
        let assignable_scopes_binding_1 = args.assignable_scopes.get_output(context);
        let assignable_scopes_binding = assignable_scopes_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let permissions_binding_1 = args.permissions.get_output(context);
        let permissions_binding = permissions_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let role_definition_id_binding_1 = args.role_definition_id.get_output(context);
        let role_definition_id_binding = role_definition_id_binding_1.get_inner();
        let type__binding_1 = args.type_.get_output(context);
        let type__binding = type__binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/sqlRoleDefinition:SqlRoleDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "assignableScopes".into(),
                    value: &assignable_scopes_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "roleDefinitionId".into(),
                    value: &role_definition_id_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SqlRoleDefinitionResult {
            account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            assignable_scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assignableScopes"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            role_definition_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleDefinitionId"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}

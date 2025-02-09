/// Manages a Cosmos DB SQL Role Assignment.
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
///       name: examplesqlroledef
///       resourceGroupName: ${example.name}
///       accountName: ${exampleAccount.name}
///       type: CustomRole
///       assignableScopes:
///         - ${exampleAccount.id}
///       permissions:
///         - dataActions:
///             - Microsoft.DocumentDB/databaseAccounts/sqlDatabases/containers/items/read
///   exampleSqlRoleAssignment:
///     type: azure:cosmosdb:SqlRoleAssignment
///     name: example
///     properties:
///       name: 736180af-7fbc-4c7f-9004-22735173c1c3
///       resourceGroupName: ${example.name}
///       accountName: ${exampleAccount.name}
///       roleDefinitionId: ${exampleSqlRoleDefinition.id}
///       principalId: ${current.objectId}
///       scope: ${exampleAccount.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Cosmos DB SQL Role Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/sqlRoleAssignment:SqlRoleAssignment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DocumentDB/databaseAccounts/account1/sqlRoleAssignments/9e007587-dbcd-4190-84cb-fcab5a09ca39
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sql_role_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlRoleAssignmentArgs {
        /// The name of the Cosmos DB Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The GUID as the name of the Cosmos DB SQL Role Assignment - one will be generated if not specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Principal (Client) in Azure Active Directory. Changing this forces a new resource to be created.
        #[builder(into)]
        pub principal_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Cosmos DB SQL Role Assignment is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the Cosmos DB SQL Role Definition.
        #[builder(into)]
        pub role_definition_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The data plane resource path for which access is being granted through this Cosmos DB SQL Role Assignment. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SqlRoleAssignmentResult {
        /// The name of the Cosmos DB Account. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// The GUID as the name of the Cosmos DB SQL Role Assignment - one will be generated if not specified. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Principal (Client) in Azure Active Directory. Changing this forces a new resource to be created.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the Cosmos DB SQL Role Assignment is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the Cosmos DB SQL Role Definition.
        pub role_definition_id: pulumi_gestalt_rust::Output<String>,
        /// The data plane resource path for which access is being granted through this Cosmos DB SQL Role Assignment. Changing this forces a new resource to be created.
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SqlRoleAssignmentArgs,
    ) -> SqlRoleAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_name_binding_1 = args.account_name.get_output(context);
        let account_name_binding = account_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let principal_id_binding_1 = args.principal_id.get_output(context);
        let principal_id_binding = principal_id_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let role_definition_id_binding_1 = args.role_definition_id.get_output(context);
        let role_definition_id_binding = role_definition_id_binding_1.get_inner();
        let scope_binding_1 = args.scope.get_output(context);
        let scope_binding = scope_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/sqlRoleAssignment:SqlRoleAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding,
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
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SqlRoleAssignmentResult {
            account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            principal_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principalId"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            role_definition_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleDefinitionId"),
            ),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
        }
    }
}

/// Assigns a given Principal (User or Group) to a given Role.
///
/// ## Example Usage
///
/// ### Using A Built-In Role)
///
/// ```yaml
/// resources:
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${primary.id}
///       roleDefinitionName: Reader
///       principalId: ${example.objectId}
/// variables:
///   primary:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
///
/// ### Custom Role & Service Principal)
///
/// ```yaml
/// resources:
///   exampleRoleDefinition:
///     type: azure:authorization:RoleDefinition
///     name: example
///     properties:
///       roleDefinitionId: 00000000-0000-0000-0000-000000000000
///       name: my-custom-role-definition
///       scope: ${primary.id}
///       permissions:
///         - actions:
///             - Microsoft.Resources/subscriptions/resourceGroups/read
///           notActions: []
///       assignableScopes:
///         - ${primary.id}
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       name: 00000000-0000-0000-0000-000000000000
///       scope: ${primary.id}
///       roleDefinitionId: ${exampleRoleDefinition.roleDefinitionResourceId}
///       principalId: ${example.objectId}
/// variables:
///   primary:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
///
/// ### Custom Role & User)
///
/// ```yaml
/// resources:
///   exampleRoleDefinition:
///     type: azure:authorization:RoleDefinition
///     name: example
///     properties:
///       roleDefinitionId: 00000000-0000-0000-0000-000000000000
///       name: my-custom-role-definition
///       scope: ${primary.id}
///       permissions:
///         - actions:
///             - Microsoft.Resources/subscriptions/resourceGroups/read
///           notActions: []
///       assignableScopes:
///         - ${primary.id}
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       name: 00000000-0000-0000-0000-000000000000
///       scope: ${primary.id}
///       roleDefinitionId: ${exampleRoleDefinition.roleDefinitionResourceId}
///       principalId: ${example.objectId}
/// variables:
///   primary:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
///
/// ### Custom Role & Management Group)
///
/// ```yaml
/// resources:
///   exampleRoleDefinition:
///     type: azure:authorization:RoleDefinition
///     name: example
///     properties:
///       roleDefinitionId: 00000000-0000-0000-0000-000000000000
///       name: my-custom-role-definition
///       scope: ${primary.id}
///       permissions:
///         - actions:
///             - Microsoft.Resources/subscriptions/resourceGroups/read
///           notActions: []
///       assignableScopes:
///         - ${primary.id}
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       name: 00000000-0000-0000-0000-000000000000
///       scope: ${primaryAzurermManagementGroup.id}
///       roleDefinitionId: ${exampleRoleDefinition.roleDefinitionResourceId}
///       principalId: ${example.objectId}
/// variables:
///   primary:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   exampleGetGroup:
///     fn::invoke:
///       function: azure:management:getGroup
///       arguments:
///         name: 00000000-0000-0000-0000-000000000000
/// ```
///
///
/// ### ABAC Condition)
///
/// ```yaml
/// resources:
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       roleDefinitionName: Role Based Access Control Administrator
///       scope: ${primary.id}
///       principalId: ${example.objectId}
///       principalType: ServicePrincipal
///       description: Role Based Access Control Administrator role assignment with ABAC Condition.
///       conditionVersion: '2.0'
///       condition:
///         fn::join:
///           - ""
///           - - |-
///               (
///                (
///                 !(ActionMatches{'Microsoft.Authorization/roleAssignments/write'})
///                )
///                OR
///                (
///                 @Request[Microsoft.Authorization/roleAssignments:RoleDefinitionId] ForAnyOfAnyValues:GuidEquals {
///             - fn::invoke:
///                 function: std:basename
///                 arguments:
///                   input: ${builtin.roleDefinitionId}
///                 return: result
///             - |-
///               }
///                )
///               )
///               AND
///               (
///                (
///                 !(ActionMatches{'Microsoft.Authorization/roleAssignments/delete'})
///                )
///                OR
///                (
///                 @Resource[Microsoft.Authorization/roleAssignments:RoleDefinitionId] ForAnyOfAnyValues:GuidEquals {
///             - fn::invoke:
///                 function: std:basename
///                 arguments:
///                   input: ${builtin.roleDefinitionId}
///                 return: result
///             - |
///               }
///                )
///               )
/// variables:
///   primary:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   builtin:
///     fn::invoke:
///       function: azure:authorization:getRoleDefinition
///       arguments:
///         name: Reader
/// ```
///
/// ## Import
///
/// Role Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:authorization/assignment:Assignment example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Authorization/roleAssignments/00000000-0000-0000-0000-000000000000
/// ```
///
/// * for scope `Subscription`, the id format is `/subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Authorization/roleAssignments/00000000-0000-0000-0000-000000000000`
///
/// * for scope `Resource Group`, the id format is `/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Authorization/roleAssignments/00000000-0000-0000-0000-000000000000`
///
/// * for scope referencing a Key Vault, the id format is `/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.KeyVault/vaults/vaultname/providers/Microsoft.Authorization/roleAssignments/00000000-0000-0000-0000-000000000000`
///
/// text
///
/// /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Authorization/roleAssignments/00000000-0000-0000-0000-000000000000|00000000-0000-0000-0000-000000000000
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssignmentArgs {
        /// The condition that limits the resources that the role can be assigned to. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The version of the condition. Possible values are `1.0` or `2.0`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub condition_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The delegated Azure Resource Id which contains a Managed Identity. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** this field is only used in cross tenant scenario.
        #[builder(into, default)]
        pub delegated_managed_identity_resource_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The description for this Role Assignment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique UUID/GUID for this Role Assignment - one will be generated if not specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Principal (User, Group or Service Principal) to assign the Role Definition to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The Principal ID is also known as the Object ID (ie not the "Application ID" for applications).
        #[builder(into)]
        pub principal_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the `principal_id`. Possible values are `User`, `Group` and `ServicePrincipal`. Changing this forces a new resource to be created. It is necessary to explicitly set this attribute when creating role assignments if the principal creating the assignment is constrained by ABAC rules that filters on the PrincipalType attribute.
        ///
        /// > **NOTE:** If one of `condition` or `condition_version` is set both fields must be present.
        #[builder(into, default)]
        pub principal_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Scoped-ID of the Role Definition. Changing this forces a new resource to be created. Conflicts with `role_definition_name`.
        #[builder(into, default)]
        pub role_definition_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of a built-in Role. Changing this forces a new resource to be created. Conflicts with `role_definition_id`.
        #[builder(into, default)]
        pub role_definition_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The scope at which the Role Assignment applies to, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`, or `/providers/Microsoft.Management/managementGroups/myMG`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If the `principal_id` is a newly provisioned `Service Principal` set this value to `true` to skip the `Azure Active Directory` check which may fail due to replication lag. This argument is only valid if the `principal_id` is a `Service Principal` identity. Defaults to `false`.
        ///
        /// > **NOTE:** If it is not a `Service Principal` identity it will cause the role assignment to fail.
        #[builder(into, default)]
        pub skip_service_principal_aad_check: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct AssignmentResult {
        /// The condition that limits the resources that the role can be assigned to. Changing this forces a new resource to be created.
        pub condition: pulumi_gestalt_rust::Output<Option<String>>,
        /// The version of the condition. Possible values are `1.0` or `2.0`. Changing this forces a new resource to be created.
        pub condition_version: pulumi_gestalt_rust::Output<String>,
        /// The delegated Azure Resource Id which contains a Managed Identity. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** this field is only used in cross tenant scenario.
        pub delegated_managed_identity_resource_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The description for this Role Assignment. Changing this forces a new resource to be created.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A unique UUID/GUID for this Role Assignment - one will be generated if not specified. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Principal (User, Group or Service Principal) to assign the Role Definition to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The Principal ID is also known as the Object ID (ie not the "Application ID" for applications).
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        /// The type of the `principal_id`. Possible values are `User`, `Group` and `ServicePrincipal`. Changing this forces a new resource to be created. It is necessary to explicitly set this attribute when creating role assignments if the principal creating the assignment is constrained by ABAC rules that filters on the PrincipalType attribute.
        ///
        /// > **NOTE:** If one of `condition` or `condition_version` is set both fields must be present.
        pub principal_type: pulumi_gestalt_rust::Output<String>,
        /// The Scoped-ID of the Role Definition. Changing this forces a new resource to be created. Conflicts with `role_definition_name`.
        pub role_definition_id: pulumi_gestalt_rust::Output<String>,
        /// The name of a built-in Role. Changing this forces a new resource to be created. Conflicts with `role_definition_id`.
        pub role_definition_name: pulumi_gestalt_rust::Output<String>,
        /// The scope at which the Role Assignment applies to, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`, or `/providers/Microsoft.Management/managementGroups/myMG`. Changing this forces a new resource to be created.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// If the `principal_id` is a newly provisioned `Service Principal` set this value to `true` to skip the `Azure Active Directory` check which may fail due to replication lag. This argument is only valid if the `principal_id` is a `Service Principal` identity. Defaults to `false`.
        ///
        /// > **NOTE:** If it is not a `Service Principal` identity it will cause the role assignment to fail.
        pub skip_service_principal_aad_check: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssignmentArgs,
    ) -> AssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let condition_version_binding = args.condition_version.get_output(context);
        let delegated_managed_identity_resource_id_binding = args
            .delegated_managed_identity_resource_id
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let principal_id_binding = args.principal_id.get_output(context);
        let principal_type_binding = args.principal_type.get_output(context);
        let role_definition_id_binding = args.role_definition_id.get_output(context);
        let role_definition_name_binding = args.role_definition_name.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let skip_service_principal_aad_check_binding = args
            .skip_service_principal_aad_check
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:authorization/assignment:Assignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: condition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "conditionVersion".into(),
                    value: condition_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegatedManagedIdentityResourceId".into(),
                    value: delegated_managed_identity_resource_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalId".into(),
                    value: principal_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalType".into(),
                    value: principal_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleDefinitionId".into(),
                    value: role_definition_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleDefinitionName".into(),
                    value: role_definition_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipServicePrincipalAadCheck".into(),
                    value: skip_service_principal_aad_check_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AssignmentResult {
            condition: o.get_field("condition"),
            condition_version: o.get_field("conditionVersion"),
            delegated_managed_identity_resource_id: o
                .get_field("delegatedManagedIdentityResourceId"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            principal_id: o.get_field("principalId"),
            principal_type: o.get_field("principalType"),
            role_definition_id: o.get_field("roleDefinitionId"),
            role_definition_name: o.get_field("roleDefinitionName"),
            scope: o.get_field("scope"),
            skip_service_principal_aad_check: o.get_field("skipServicePrincipalAadCheck"),
        }
    }
}

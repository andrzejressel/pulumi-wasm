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
pub mod assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssignmentArgs {
        /// The condition that limits the resources that the role can be assigned to. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<Option<String>>,
        /// The version of the condition. Possible values are `1.0` or `2.0`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub condition_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The delegated Azure Resource Id which contains a Managed Identity. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** this field is only used in cross tenant scenario.
        #[builder(into, default)]
        pub delegated_managed_identity_resource_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The description for this Role Assignment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique UUID/GUID for this Role Assignment - one will be generated if not specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Principal (User, Group or Service Principal) to assign the Role Definition to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The Principal ID is also known as the Object ID (ie not the "Application ID" for applications).
        #[builder(into)]
        pub principal_id: pulumi_wasm_rust::Output<String>,
        /// The type of the `principal_id`. Possible values are `User`, `Group` and `ServicePrincipal`. Changing this forces a new resource to be created. It is necessary to explicitly set this attribute when creating role assignments if the principal creating the assignment is constrained by ABAC rules that filters on the PrincipalType attribute.
        ///
        /// > **NOTE:** If one of `condition` or `condition_version` is set both fields must be present.
        #[builder(into, default)]
        pub principal_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The Scoped-ID of the Role Definition. Changing this forces a new resource to be created. Conflicts with `role_definition_name`.
        #[builder(into, default)]
        pub role_definition_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of a built-in Role. Changing this forces a new resource to be created. Conflicts with `role_definition_id`.
        #[builder(into, default)]
        pub role_definition_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The scope at which the Role Assignment applies to, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`, or `/providers/Microsoft.Management/managementGroups/myMG`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<String>,
        /// If the `principal_id` is a newly provisioned `Service Principal` set this value to `true` to skip the `Azure Active Directory` check which may fail due to replication lag. This argument is only valid if the `principal_id` is a `Service Principal` identity. Defaults to `false`.
        ///
        /// > **NOTE:** If it is not a `Service Principal` identity it will cause the role assignment to fail.
        #[builder(into, default)]
        pub skip_service_principal_aad_check: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct AssignmentResult {
        /// The condition that limits the resources that the role can be assigned to. Changing this forces a new resource to be created.
        pub condition: pulumi_wasm_rust::Output<Option<String>>,
        /// The version of the condition. Possible values are `1.0` or `2.0`. Changing this forces a new resource to be created.
        pub condition_version: pulumi_wasm_rust::Output<String>,
        /// The delegated Azure Resource Id which contains a Managed Identity. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** this field is only used in cross tenant scenario.
        pub delegated_managed_identity_resource_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The description for this Role Assignment. Changing this forces a new resource to be created.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique UUID/GUID for this Role Assignment - one will be generated if not specified. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Principal (User, Group or Service Principal) to assign the Role Definition to. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The Principal ID is also known as the Object ID (ie not the "Application ID" for applications).
        pub principal_id: pulumi_wasm_rust::Output<String>,
        /// The type of the `principal_id`. Possible values are `User`, `Group` and `ServicePrincipal`. Changing this forces a new resource to be created. It is necessary to explicitly set this attribute when creating role assignments if the principal creating the assignment is constrained by ABAC rules that filters on the PrincipalType attribute.
        ///
        /// > **NOTE:** If one of `condition` or `condition_version` is set both fields must be present.
        pub principal_type: pulumi_wasm_rust::Output<String>,
        /// The Scoped-ID of the Role Definition. Changing this forces a new resource to be created. Conflicts with `role_definition_name`.
        pub role_definition_id: pulumi_wasm_rust::Output<String>,
        /// The name of a built-in Role. Changing this forces a new resource to be created. Conflicts with `role_definition_id`.
        pub role_definition_name: pulumi_wasm_rust::Output<String>,
        /// The scope at which the Role Assignment applies to, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`, or `/providers/Microsoft.Management/managementGroups/myMG`. Changing this forces a new resource to be created.
        pub scope: pulumi_wasm_rust::Output<String>,
        /// If the `principal_id` is a newly provisioned `Service Principal` set this value to `true` to skip the `Azure Active Directory` check which may fail due to replication lag. This argument is only valid if the `principal_id` is a `Service Principal` identity. Defaults to `false`.
        ///
        /// > **NOTE:** If it is not a `Service Principal` identity it will cause the role assignment to fail.
        pub skip_service_principal_aad_check: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AssignmentArgs) -> AssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let condition_version_binding = args.condition_version.get_inner();
        let delegated_managed_identity_resource_id_binding = args
            .delegated_managed_identity_resource_id
            .get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let principal_id_binding = args.principal_id.get_inner();
        let principal_type_binding = args.principal_type.get_inner();
        let role_definition_id_binding = args.role_definition_id.get_inner();
        let role_definition_name_binding = args.role_definition_name.get_inner();
        let scope_binding = args.scope.get_inner();
        let skip_service_principal_aad_check_binding = args
            .skip_service_principal_aad_check
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:authorization/assignment:Assignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "conditionVersion".into(),
                    value: &condition_version_binding,
                },
                register_interface::ObjectField {
                    name: "delegatedManagedIdentityResourceId".into(),
                    value: &delegated_managed_identity_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "principalType".into(),
                    value: &principal_type_binding,
                },
                register_interface::ObjectField {
                    name: "roleDefinitionId".into(),
                    value: &role_definition_id_binding,
                },
                register_interface::ObjectField {
                    name: "roleDefinitionName".into(),
                    value: &role_definition_name_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "skipServicePrincipalAadCheck".into(),
                    value: &skip_service_principal_aad_check_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "conditionVersion".into(),
                },
                register_interface::ResultField {
                    name: "delegatedManagedIdentityResourceId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "principalId".into(),
                },
                register_interface::ResultField {
                    name: "principalType".into(),
                },
                register_interface::ResultField {
                    name: "roleDefinitionId".into(),
                },
                register_interface::ResultField {
                    name: "roleDefinitionName".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "skipServicePrincipalAadCheck".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AssignmentResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            condition_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conditionVersion").unwrap(),
            ),
            delegated_managed_identity_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegatedManagedIdentityResourceId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            principal_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalId").unwrap(),
            ),
            principal_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalType").unwrap(),
            ),
            role_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleDefinitionId").unwrap(),
            ),
            role_definition_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleDefinitionName").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            skip_service_principal_aad_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipServicePrincipalAadCheck").unwrap(),
            ),
        }
    }
}

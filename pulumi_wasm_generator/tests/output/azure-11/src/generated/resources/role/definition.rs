/// Manages a custom Role Definition, used to assign Roles to Users/Principals. See ['Understand role definitions'](https://docs.microsoft.com/azure/role-based-access-control/role-definitions) in the Azure documentation for more details.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:authorization:RoleDefinition
///     properties:
///       name: my-custom-role
///       scope: ${primary.id}
///       description: This is a custom role created
///       permissions:
///         - actions:
///             - '*'
///           notActions: []
///       assignableScopes:
///         - ${primary.id}
/// variables:
///   primary:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
///
/// ### With Management Group
/// ```yaml
/// resources:
///   example:
///     type: azure:management:Group
///     properties:
///       displayName: ParentGroup
///       subscriptionIds:
///         - ${current.subscriptionId}
///   exampleRoleDefinition:
///     type: azure:authorization:RoleDefinition
///     name: example
///     properties:
///       name: example-mg-role
///       scope: ${example.id}
///       description: Example custom role scoped to a management group.
///       permissions:
///         - actions:
///             - Microsoft.Insights/alertRules/*
///           notActions: []
///       assignableScopes:
///         - ${example.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Role Definitions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:role/definition:Definition example "/subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Authorization/roleDefinitions/00000000-0000-0000-0000-000000000000|/subscriptions/00000000-0000-0000-0000-000000000000"
/// ```
///
pub mod definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefinitionArgs {
        /// One or more assignable scopes for this Role Definition, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, `/providers/Microsoft.Management/managementGroups/0b1f6471-1bf0-4dda-aec3-111122223333` , or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`.
        ///
        /// > **NOTE:** The value for `scope` is automatically included in this list if no other values supplied.
        #[builder(into, default)]
        pub assignable_scopes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A description of the Role Definition.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Role Definition.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `permissions` block as defined below.
        #[builder(into, default)]
        pub permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::role::DefinitionPermission>>,
        >,
        /// A unique UUID/GUID which identifies this role - one will be generated if not specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub role_definition_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The scope at which the Role Definition applies to, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, `/providers/Microsoft.Management/managementGroups/0b1f6471-1bf0-4dda-aec3-111122223333`, or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`. It is recommended to use the first entry of the `assignable_scopes`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DefinitionResult {
        /// One or more assignable scopes for this Role Definition, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, `/providers/Microsoft.Management/managementGroups/0b1f6471-1bf0-4dda-aec3-111122223333` , or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`.
        ///
        /// > **NOTE:** The value for `scope` is automatically included in this list if no other values supplied.
        pub assignable_scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// A description of the Role Definition.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Role Definition.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `permissions` block as defined below.
        pub permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::role::DefinitionPermission>>,
        >,
        /// A unique UUID/GUID which identifies this role - one will be generated if not specified. Changing this forces a new resource to be created.
        pub role_definition_id: pulumi_wasm_rust::Output<String>,
        /// The Azure Resource Manager ID for the resource.
        pub role_definition_resource_id: pulumi_wasm_rust::Output<String>,
        /// The scope at which the Role Definition applies to, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, `/providers/Microsoft.Management/managementGroups/0b1f6471-1bf0-4dda-aec3-111122223333`, or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`. It is recommended to use the first entry of the `assignable_scopes`. Changing this forces a new resource to be created.
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DefinitionArgs) -> DefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assignable_scopes_binding = args.assignable_scopes.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let permissions_binding = args.permissions.get_inner();
        let role_definition_id_binding = args.role_definition_id.get_inner();
        let scope_binding = args.scope.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:role/definition:Definition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assignableScopes".into(),
                    value: &assignable_scopes_binding,
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
                    name: "permissions".into(),
                    value: &permissions_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "assignableScopes".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "roleDefinitionId".into(),
                },
                register_interface::ResultField {
                    name: "roleDefinitionResourceId".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DefinitionResult {
            assignable_scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignableScopes").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            role_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleDefinitionId").unwrap(),
            ),
            role_definition_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleDefinitionResourceId").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
        }
    }
}

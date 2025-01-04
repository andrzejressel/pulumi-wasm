pub mod get_role_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRoleDefinitionArgs {
        /// Specifies the Name of either a built-in or custom Role Definition.
        ///
        /// > You can also use this for built-in roles such as `Contributor`, `Owner`, `Reader` and `Virtual Machine Contributor`
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Role Definition as a UUID/GUID.
        #[builder(into, default)]
        pub role_definition_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Scope at which the Custom Role Definition exists.
        ///
        /// > **Note:** One of `name` or `role_definition_id` must be specified.
        #[builder(into, default)]
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRoleDefinitionResult {
        /// One or more assignable scopes for this Role Definition, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333`, `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`, or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup/providers/Microsoft.Compute/virtualMachines/myVM`.
        pub assignable_scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Description of the built-in Role.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `permissions` block as documented below.
        pub permissions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::authorization::GetRoleDefinitionPermission>,
        >,
        pub role_definition_id: pulumi_wasm_rust::Output<String>,
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
        /// The Type of the Role.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRoleDefinitionArgs) -> GetRoleDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let role_definition_id_binding = args.role_definition_id.get_inner();
        let scope_binding = args.scope.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:authorization/getRoleDefinition:getRoleDefinition".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "id".into(),
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
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRoleDefinitionResult {
            assignable_scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignableScopes").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            role_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleDefinitionId").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}

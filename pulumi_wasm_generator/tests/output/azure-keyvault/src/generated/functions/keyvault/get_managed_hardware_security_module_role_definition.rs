pub mod get_managed_hardware_security_module_role_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleRoleDefinitionArgs {
        #[builder(into)]
        pub managed_hsm_id: pulumi_wasm_rust::Output<String>,
        /// The name in UUID notation of this KeyVault Role Definition.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleRoleDefinitionResult {
        /// A list of assignable role scope. Possible values are `/` and `/keys`.
        pub assignable_scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// A text description of this role definition.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub managed_hsm_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `permission` block as defined below.
        pub permissions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::keyvault::GetManagedHardwareSecurityModuleRoleDefinitionPermission,
            >,
        >,
        /// The ID of the role definition resource without base url.
        pub resource_manager_id: pulumi_wasm_rust::Output<String>,
        /// The role name of the role definition.
        pub role_name: pulumi_wasm_rust::Output<String>,
        /// The type of the role definition. Possible values are `AKVBuiltInRole` and `CustomRole`.
        pub role_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetManagedHardwareSecurityModuleRoleDefinitionArgs,
    ) -> GetManagedHardwareSecurityModuleRoleDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let managed_hsm_id_binding = args.managed_hsm_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getManagedHardwareSecurityModuleRoleDefinition:getManagedHardwareSecurityModuleRoleDefinition"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managedHsmId".into(),
                    value: &managed_hsm_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "managedHsmId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "resourceManagerId".into(),
                },
                register_interface::ResultField {
                    name: "roleName".into(),
                },
                register_interface::ResultField {
                    name: "roleType".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetManagedHardwareSecurityModuleRoleDefinitionResult {
            assignable_scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignableScopes").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            managed_hsm_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedHsmId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            resource_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceManagerId").unwrap(),
            ),
            role_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleName").unwrap(),
            ),
            role_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleType").unwrap(),
            ),
        }
    }
}

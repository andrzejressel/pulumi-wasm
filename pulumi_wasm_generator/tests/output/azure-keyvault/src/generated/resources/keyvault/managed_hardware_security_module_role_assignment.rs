/// Manages a Managed Hardware Security Module Role Assignment.
///
/// ## Import
///
/// Managed Hardware Security Modules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/managedHardwareSecurityModuleRoleAssignment:ManagedHardwareSecurityModuleRoleAssignment example https://0000.managedhsm.azure.net///RoleAssignment/00000000-0000-0000-0000-000000000000
/// ```
///
pub mod managed_hardware_security_module_role_assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleRoleAssignmentArgs {
        /// The ID of a Managed Hardware Security Module resource. Changing this forces a new Managed Hardware Security Module to be created.
        /// *
        #[builder(into)]
        pub managed_hsm_id: pulumi_wasm_rust::Output<String>,
        /// The name in GUID notation which should be used for this Managed Hardware Security Module Role Assignment. Changing this forces a new Managed Hardware Security Module to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The principal ID to be assigned to this role. It can point to a user, service principal, or security group. Changing this forces a new Managed Hardware Security Module to be created.
        #[builder(into)]
        pub principal_id: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the role definition to assign. Changing this forces a new Managed Hardware Security Module to be created.
        #[builder(into)]
        pub role_definition_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the scope to create the role assignment. Changing this forces a new Managed Hardware Security Module to be created.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleRoleAssignmentResult {
        /// The ID of a Managed Hardware Security Module resource. Changing this forces a new Managed Hardware Security Module to be created.
        /// *
        pub managed_hsm_id: pulumi_wasm_rust::Output<String>,
        /// The name in GUID notation which should be used for this Managed Hardware Security Module Role Assignment. Changing this forces a new Managed Hardware Security Module to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The principal ID to be assigned to this role. It can point to a user, service principal, or security group. Changing this forces a new Managed Hardware Security Module to be created.
        pub principal_id: pulumi_wasm_rust::Output<String>,
        /// (Deprecated) The resource id of created assignment resource.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the role definition to assign. Changing this forces a new Managed Hardware Security Module to be created.
        pub role_definition_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the scope to create the role assignment. Changing this forces a new Managed Hardware Security Module to be created.
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedHardwareSecurityModuleRoleAssignmentArgs,
    ) -> ManagedHardwareSecurityModuleRoleAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let managed_hsm_id_binding = args.managed_hsm_id.get_inner();
        let name_binding = args.name.get_inner();
        let principal_id_binding = args.principal_id.get_inner();
        let role_definition_id_binding = args.role_definition_id.get_inner();
        let scope_binding = args.scope.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/managedHardwareSecurityModuleRoleAssignment:ManagedHardwareSecurityModuleRoleAssignment"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managedHsmId".into(),
                    value: &managed_hsm_id_binding,
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
                    name: "managedHsmId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "principalId".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "roleDefinitionId".into(),
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
        ManagedHardwareSecurityModuleRoleAssignmentResult {
            managed_hsm_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedHsmId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            principal_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalId").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            role_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleDefinitionId").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
        }
    }
}
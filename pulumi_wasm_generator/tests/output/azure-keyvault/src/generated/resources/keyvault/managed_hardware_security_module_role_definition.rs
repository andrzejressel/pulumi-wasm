/// Manages a KeyVault Managed Hardware Security Module Role Definition. This resource works together with Managed hardware security module resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:keyvault:ManagedHardwareSecurityModule
///     properties:
///       name: example
///       resourceGroupName: ${exampleAzurermResourceGroup.name}
///       location: ${exampleAzurermResourceGroup.location}
///       skuName: Standard_B1
///       tenantId: ${current.tenantId}
///       adminObjectIds:
///         - ${current.objectId}
///       purgeProtectionEnabled: false
///       activeConfig:
///         - securityDomainCertificate:
///             - ${cert[0].id}
///             - ${cert[1].id}
///             - ${cert[2].id}
///           securityDomainQuorum: 2
///   exampleManagedHardwareSecurityModuleRoleDefinition:
///     type: azure:keyvault:ManagedHardwareSecurityModuleRoleDefinition
///     name: example
///     properties:
///       name: 7d206142-bf01-11ed-80bc-00155d61ee9e
///       vaultBaseUrl: ${example.hsmUri}
///       description: desc foo
///       permissions:
///         - dataActions:
///             - Microsoft.KeyVault/managedHsm/keys/read/action
/// ```
///
/// ## Import
///
/// KeyVaults can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/managedHardwareSecurityModuleRoleDefinition:ManagedHardwareSecurityModuleRoleDefinition example https://0000.managedhsm.azure.net///RoleDefinition/00000000-0000-0000-0000-000000000000
/// ```
///
pub mod managed_hardware_security_module_role_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleRoleDefinitionArgs {
        /// Specifies a text description about this KeyVault Role Definition.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into)]
        pub managed_hsm_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this KeyVault Role Definition. Changing this forces a new KeyVault Role Definition to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `permission` blocks as defined below.
        #[builder(into, default)]
        pub permissions: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::keyvault::ManagedHardwareSecurityModuleRoleDefinitionPermission,
                >,
            >,
        >,
        /// Specify a name for this KeyVault Role Definition.
        #[builder(into, default)]
        pub role_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleRoleDefinitionResult {
        /// Specifies a text description about this KeyVault Role Definition.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub managed_hsm_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this KeyVault Role Definition. Changing this forces a new KeyVault Role Definition to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `permission` blocks as defined below.
        pub permissions: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::keyvault::ManagedHardwareSecurityModuleRoleDefinitionPermission,
                >,
            >,
        >,
        /// The ID of the role definition resource without Key Vault base URL.
        pub resource_manager_id: pulumi_wasm_rust::Output<String>,
        /// Specify a name for this KeyVault Role Definition.
        pub role_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of the role definition. Possible values are `AKVBuiltInRole` and `CustomRole`.
        pub role_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedHardwareSecurityModuleRoleDefinitionArgs,
    ) -> ManagedHardwareSecurityModuleRoleDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let managed_hsm_id_binding = args.managed_hsm_id.get_inner();
        let name_binding = args.name.get_inner();
        let permissions_binding = args.permissions.get_inner();
        let role_name_binding = args.role_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/managedHardwareSecurityModuleRoleDefinition:ManagedHardwareSecurityModuleRoleDefinition"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "managedHsmId".into(),
                    value: &managed_hsm_id_binding,
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
                    name: "roleName".into(),
                    value: &role_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedHardwareSecurityModuleRoleDefinitionResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
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
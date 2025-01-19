/// Manages an association of a Resource Guard and Recovery Services Vault.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleResourceGuard = resource_guard::create(
///         "exampleResourceGuard",
///         ResourceGuardArgs::builder()
///             .location("${example.location}")
///             .name("example-resourceguard")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let test = vault_resource_guard_association::create(
///         "test",
///         VaultResourceGuardAssociationArgs::builder()
///             .resource_guard_id("${testAzurermDataProtectionResourceGuard.id}")
///             .vault_id("${testAzurermRecoveryServicesVault.id}")
///             .build_struct(),
///     );
///     let vault = vault::create(
///         "vault",
///         VaultArgs::builder()
///             .location("${example.location}")
///             .name("example-recovery-vault")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .soft_delete_enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Resource Guards can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:recoveryservices/vaultResourceGuardAssociation:VaultResourceGuardAssociation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.RecoveryServices/vaults/vault1/backupResourceGuardProxies/proxy1
/// ```
///
pub mod vault_resource_guard_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultResourceGuardAssociationArgs {
        /// ID of the Resource Guard which should be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_guard_id: pulumi_wasm_rust::Output<String>,
        /// ID of the Recovery Services Vault which should be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vault_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VaultResourceGuardAssociationResult {
        /// ID of the Resource Guard which should be associated with. Changing this forces a new resource to be created.
        pub resource_guard_id: pulumi_wasm_rust::Output<String>,
        /// ID of the Recovery Services Vault which should be associated with. Changing this forces a new resource to be created.
        pub vault_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VaultResourceGuardAssociationArgs,
    ) -> VaultResourceGuardAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_guard_id_binding = args.resource_guard_id.get_inner();
        let vault_id_binding = args.vault_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:recoveryservices/vaultResourceGuardAssociation:VaultResourceGuardAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourceGuardId".into(),
                    value: &resource_guard_id_binding,
                },
                register_interface::ObjectField {
                    name: "vaultId".into(),
                    value: &vault_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "resourceGuardId".into(),
                },
                register_interface::ResultField {
                    name: "vaultId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VaultResourceGuardAssociationResult {
            resource_guard_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGuardId").unwrap(),
            ),
            vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vaultId").unwrap(),
            ),
        }
    }
}

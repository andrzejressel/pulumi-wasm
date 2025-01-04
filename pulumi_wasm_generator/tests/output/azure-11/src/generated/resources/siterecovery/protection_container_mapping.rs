/// Manages a Azure recovery vault protection container mapping. A protection container mapping decides how to translate the protection container when a VM is migrated from one region to another.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Site Recovery Protection Container Mappings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/protectionContainerMapping:ProtectionContainerMapping mymapping /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationFabrics/fabric1/replicationProtectionContainers/container1/replicationProtectionContainerMappings/mapping1
/// ```
///
pub mod protection_container_mapping {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectionContainerMappingArgs {
        /// a `automatic_update` block defined as below.
        #[builder(into, default)]
        pub automatic_update: pulumi_wasm_rust::Output<
            Option<
                super::super::types::siterecovery::ProtectionContainerMappingAutomaticUpdate,
            >,
        >,
        /// The name of the protection container mapping. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of fabric that should contains the protection container to map. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_fabric_name: pulumi_wasm_rust::Output<String>,
        /// Id of the policy to use for this mapping. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_replication_policy_id: pulumi_wasm_rust::Output<String>,
        /// Name of the source protection container to map. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_source_protection_container_name: pulumi_wasm_rust::Output<String>,
        /// Id of target protection container to map to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_target_protection_container_id: pulumi_wasm_rust::Output<String>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_wasm_rust::Output<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ProtectionContainerMappingResult {
        /// a `automatic_update` block defined as below.
        pub automatic_update: pulumi_wasm_rust::Output<
            super::super::types::siterecovery::ProtectionContainerMappingAutomaticUpdate,
        >,
        /// The name of the protection container mapping. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Name of fabric that should contains the protection container to map. Changing this forces a new resource to be created.
        pub recovery_fabric_name: pulumi_wasm_rust::Output<String>,
        /// Id of the policy to use for this mapping. Changing this forces a new resource to be created.
        pub recovery_replication_policy_id: pulumi_wasm_rust::Output<String>,
        /// Name of the source protection container to map. Changing this forces a new resource to be created.
        pub recovery_source_protection_container_name: pulumi_wasm_rust::Output<String>,
        /// Id of target protection container to map to. Changing this forces a new resource to be created.
        pub recovery_target_protection_container_id: pulumi_wasm_rust::Output<String>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_wasm_rust::Output<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ProtectionContainerMappingArgs,
    ) -> ProtectionContainerMappingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automatic_update_binding = args.automatic_update.get_inner();
        let name_binding = args.name.get_inner();
        let recovery_fabric_name_binding = args.recovery_fabric_name.get_inner();
        let recovery_replication_policy_id_binding = args
            .recovery_replication_policy_id
            .get_inner();
        let recovery_source_protection_container_name_binding = args
            .recovery_source_protection_container_name
            .get_inner();
        let recovery_target_protection_container_id_binding = args
            .recovery_target_protection_container_id
            .get_inner();
        let recovery_vault_name_binding = args.recovery_vault_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:siterecovery/protectionContainerMapping:ProtectionContainerMapping"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automaticUpdate".into(),
                    value: &automatic_update_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryFabricName".into(),
                    value: &recovery_fabric_name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryReplicationPolicyId".into(),
                    value: &recovery_replication_policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "recoverySourceProtectionContainerName".into(),
                    value: &recovery_source_protection_container_name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryTargetProtectionContainerId".into(),
                    value: &recovery_target_protection_container_id_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryVaultName".into(),
                    value: &recovery_vault_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "automaticUpdate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recoveryFabricName".into(),
                },
                register_interface::ResultField {
                    name: "recoveryReplicationPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "recoverySourceProtectionContainerName".into(),
                },
                register_interface::ResultField {
                    name: "recoveryTargetProtectionContainerId".into(),
                },
                register_interface::ResultField {
                    name: "recoveryVaultName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProtectionContainerMappingResult {
            automatic_update: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automaticUpdate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recovery_fabric_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryFabricName").unwrap(),
            ),
            recovery_replication_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryReplicationPolicyId").unwrap(),
            ),
            recovery_source_protection_container_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoverySourceProtectionContainerName").unwrap(),
            ),
            recovery_target_protection_container_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryTargetProtectionContainerId").unwrap(),
            ),
            recovery_vault_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryVaultName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}

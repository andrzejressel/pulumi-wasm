/// Manages a Site Recovery Replication Recovery Plan within a Recovery Services vault. A recovery plan gathers machines into recovery groups for the purpose of failover.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Site Recovery Fabric can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/replicationRecoveryPlan:ReplicationRecoveryPlan azurerm_site_recovery_replication_recovery_plan.example /subscriptions/00000000-0000-0000-0000-00000000000/resourceGroups/groupName/providers/Microsoft.RecoveryServices/vaults/vaultName/replicationRecoveryPlans/planName
/// ```
///
pub mod replication_recovery_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationRecoveryPlanArgs {
        /// An `azure_to_azure_settings` block as defined below.
        #[builder(into, default)]
        pub azure_to_azure_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::siterecovery::ReplicationRecoveryPlanAzureToAzureSettings,
            >,
        >,
        /// One or more `boot_recovery_group` blocks as defined below.
        ///
        /// > **NOTE:** At least one `boot_recovery_group` block will be required in the next major version of the AzureRM Provider.
        #[builder(into)]
        pub boot_recovery_groups: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::siterecovery::ReplicationRecoveryPlanBootRecoveryGroup,
            >,
        >,
        /// One `failover_recovery_group` block as defined below.
        ///
        /// > **NOTE:** `failover_recovery_group` will be required in the next major version of the AzureRM Provider.
        #[builder(into)]
        pub failover_recovery_group: pulumi_wasm_rust::Output<
            super::super::types::siterecovery::ReplicationRecoveryPlanFailoverRecoveryGroup,
        >,
        /// The name of the Replication Plan. The name can contain only letters, numbers, and hyphens. It should start with a letter and end with a letter or a number. Can be a maximum of 63 characters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the vault that should be updated. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_id: pulumi_wasm_rust::Output<String>,
        /// One `shutdown_recovery_group` block as defined below.
        ///
        /// > **NOTE:** `shutdown_recovery_group` will be required in the next major version of the AzureRM Provider.
        #[builder(into)]
        pub shutdown_recovery_group: pulumi_wasm_rust::Output<
            super::super::types::siterecovery::ReplicationRecoveryPlanShutdownRecoveryGroup,
        >,
        /// ID of source fabric to be recovered from. Changing this forces a new Replication Plan to be created.
        #[builder(into)]
        pub source_recovery_fabric_id: pulumi_wasm_rust::Output<String>,
        /// ID of target fabric to recover. Changing this forces a new Replication Plan to be created.
        #[builder(into)]
        pub target_recovery_fabric_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ReplicationRecoveryPlanResult {
        /// An `azure_to_azure_settings` block as defined below.
        pub azure_to_azure_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::siterecovery::ReplicationRecoveryPlanAzureToAzureSettings,
            >,
        >,
        /// One or more `boot_recovery_group` blocks as defined below.
        ///
        /// > **NOTE:** At least one `boot_recovery_group` block will be required in the next major version of the AzureRM Provider.
        pub boot_recovery_groups: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::siterecovery::ReplicationRecoveryPlanBootRecoveryGroup,
            >,
        >,
        /// One `failover_recovery_group` block as defined below.
        ///
        /// > **NOTE:** `failover_recovery_group` will be required in the next major version of the AzureRM Provider.
        pub failover_recovery_group: pulumi_wasm_rust::Output<
            super::super::types::siterecovery::ReplicationRecoveryPlanFailoverRecoveryGroup,
        >,
        /// The name of the Replication Plan. The name can contain only letters, numbers, and hyphens. It should start with a letter and end with a letter or a number. Can be a maximum of 63 characters. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the vault that should be updated. Changing this forces a new resource to be created.
        pub recovery_vault_id: pulumi_wasm_rust::Output<String>,
        /// One `shutdown_recovery_group` block as defined below.
        ///
        /// > **NOTE:** `shutdown_recovery_group` will be required in the next major version of the AzureRM Provider.
        pub shutdown_recovery_group: pulumi_wasm_rust::Output<
            super::super::types::siterecovery::ReplicationRecoveryPlanShutdownRecoveryGroup,
        >,
        /// ID of source fabric to be recovered from. Changing this forces a new Replication Plan to be created.
        pub source_recovery_fabric_id: pulumi_wasm_rust::Output<String>,
        /// ID of target fabric to recover. Changing this forces a new Replication Plan to be created.
        pub target_recovery_fabric_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ReplicationRecoveryPlanArgs,
    ) -> ReplicationRecoveryPlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let azure_to_azure_settings_binding = args.azure_to_azure_settings.get_inner();
        let boot_recovery_groups_binding = args.boot_recovery_groups.get_inner();
        let failover_recovery_group_binding = args.failover_recovery_group.get_inner();
        let name_binding = args.name.get_inner();
        let recovery_vault_id_binding = args.recovery_vault_id.get_inner();
        let shutdown_recovery_group_binding = args.shutdown_recovery_group.get_inner();
        let source_recovery_fabric_id_binding = args
            .source_recovery_fabric_id
            .get_inner();
        let target_recovery_fabric_id_binding = args
            .target_recovery_fabric_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:siterecovery/replicationRecoveryPlan:ReplicationRecoveryPlan"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "azureToAzureSettings".into(),
                    value: &azure_to_azure_settings_binding,
                },
                register_interface::ObjectField {
                    name: "bootRecoveryGroups".into(),
                    value: &boot_recovery_groups_binding,
                },
                register_interface::ObjectField {
                    name: "failoverRecoveryGroup".into(),
                    value: &failover_recovery_group_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryVaultId".into(),
                    value: &recovery_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "shutdownRecoveryGroup".into(),
                    value: &shutdown_recovery_group_binding,
                },
                register_interface::ObjectField {
                    name: "sourceRecoveryFabricId".into(),
                    value: &source_recovery_fabric_id_binding,
                },
                register_interface::ObjectField {
                    name: "targetRecoveryFabricId".into(),
                    value: &target_recovery_fabric_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "azureToAzureSettings".into(),
                },
                register_interface::ResultField {
                    name: "bootRecoveryGroups".into(),
                },
                register_interface::ResultField {
                    name: "failoverRecoveryGroup".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recoveryVaultId".into(),
                },
                register_interface::ResultField {
                    name: "shutdownRecoveryGroup".into(),
                },
                register_interface::ResultField {
                    name: "sourceRecoveryFabricId".into(),
                },
                register_interface::ResultField {
                    name: "targetRecoveryFabricId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReplicationRecoveryPlanResult {
            azure_to_azure_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureToAzureSettings").unwrap(),
            ),
            boot_recovery_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootRecoveryGroups").unwrap(),
            ),
            failover_recovery_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failoverRecoveryGroup").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recovery_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryVaultId").unwrap(),
            ),
            shutdown_recovery_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shutdownRecoveryGroup").unwrap(),
            ),
            source_recovery_fabric_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceRecoveryFabricId").unwrap(),
            ),
            target_recovery_fabric_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetRecoveryFabricId").unwrap(),
            ),
        }
    }
}

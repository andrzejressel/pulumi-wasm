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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod replication_recovery_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationRecoveryPlanArgs {
        /// An `azure_to_azure_settings` block as defined below.
        #[builder(into, default)]
        pub azure_to_azure_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::siterecovery::ReplicationRecoveryPlanAzureToAzureSettings,
            >,
        >,
        /// One or more `boot_recovery_group` blocks as defined below.
        ///
        /// > **NOTE:** At least one `boot_recovery_group` block will be required in the next major version of the AzureRM Provider.
        #[builder(into)]
        pub boot_recovery_groups: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::siterecovery::ReplicationRecoveryPlanBootRecoveryGroup,
            >,
        >,
        /// One `failover_recovery_group` block as defined below.
        ///
        /// > **NOTE:** `failover_recovery_group` will be required in the next major version of the AzureRM Provider.
        #[builder(into)]
        pub failover_recovery_group: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::siterecovery::ReplicationRecoveryPlanFailoverRecoveryGroup,
        >,
        /// The name of the Replication Plan. The name can contain only letters, numbers, and hyphens. It should start with a letter and end with a letter or a number. Can be a maximum of 63 characters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the vault that should be updated. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One `shutdown_recovery_group` block as defined below.
        ///
        /// > **NOTE:** `shutdown_recovery_group` will be required in the next major version of the AzureRM Provider.
        #[builder(into)]
        pub shutdown_recovery_group: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::siterecovery::ReplicationRecoveryPlanShutdownRecoveryGroup,
        >,
        /// ID of source fabric to be recovered from. Changing this forces a new Replication Plan to be created.
        #[builder(into)]
        pub source_recovery_fabric_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of target fabric to recover. Changing this forces a new Replication Plan to be created.
        #[builder(into)]
        pub target_recovery_fabric_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReplicationRecoveryPlanResult {
        /// An `azure_to_azure_settings` block as defined below.
        pub azure_to_azure_settings: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::siterecovery::ReplicationRecoveryPlanAzureToAzureSettings,
            >,
        >,
        /// One or more `boot_recovery_group` blocks as defined below.
        ///
        /// > **NOTE:** At least one `boot_recovery_group` block will be required in the next major version of the AzureRM Provider.
        pub boot_recovery_groups: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::siterecovery::ReplicationRecoveryPlanBootRecoveryGroup,
            >,
        >,
        /// One `failover_recovery_group` block as defined below.
        ///
        /// > **NOTE:** `failover_recovery_group` will be required in the next major version of the AzureRM Provider.
        pub failover_recovery_group: pulumi_gestalt_rust::Output<
            super::super::types::siterecovery::ReplicationRecoveryPlanFailoverRecoveryGroup,
        >,
        /// The name of the Replication Plan. The name can contain only letters, numbers, and hyphens. It should start with a letter and end with a letter or a number. Can be a maximum of 63 characters. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the vault that should be updated. Changing this forces a new resource to be created.
        pub recovery_vault_id: pulumi_gestalt_rust::Output<String>,
        /// One `shutdown_recovery_group` block as defined below.
        ///
        /// > **NOTE:** `shutdown_recovery_group` will be required in the next major version of the AzureRM Provider.
        pub shutdown_recovery_group: pulumi_gestalt_rust::Output<
            super::super::types::siterecovery::ReplicationRecoveryPlanShutdownRecoveryGroup,
        >,
        /// ID of source fabric to be recovered from. Changing this forces a new Replication Plan to be created.
        pub source_recovery_fabric_id: pulumi_gestalt_rust::Output<String>,
        /// ID of target fabric to recover. Changing this forces a new Replication Plan to be created.
        pub target_recovery_fabric_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReplicationRecoveryPlanArgs,
    ) -> ReplicationRecoveryPlanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let azure_to_azure_settings_binding = args
            .azure_to_azure_settings
            .get_output(context);
        let boot_recovery_groups_binding = args.boot_recovery_groups.get_output(context);
        let failover_recovery_group_binding = args
            .failover_recovery_group
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let recovery_vault_id_binding = args.recovery_vault_id.get_output(context);
        let shutdown_recovery_group_binding = args
            .shutdown_recovery_group
            .get_output(context);
        let source_recovery_fabric_id_binding = args
            .source_recovery_fabric_id
            .get_output(context);
        let target_recovery_fabric_id_binding = args
            .target_recovery_fabric_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:siterecovery/replicationRecoveryPlan:ReplicationRecoveryPlan"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureToAzureSettings".into(),
                    value: azure_to_azure_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bootRecoveryGroups".into(),
                    value: boot_recovery_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "failoverRecoveryGroup".into(),
                    value: failover_recovery_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryVaultId".into(),
                    value: recovery_vault_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shutdownRecoveryGroup".into(),
                    value: shutdown_recovery_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceRecoveryFabricId".into(),
                    value: source_recovery_fabric_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetRecoveryFabricId".into(),
                    value: target_recovery_fabric_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReplicationRecoveryPlanResult {
            azure_to_azure_settings: o.get_field("azureToAzureSettings"),
            boot_recovery_groups: o.get_field("bootRecoveryGroups"),
            failover_recovery_group: o.get_field("failoverRecoveryGroup"),
            name: o.get_field("name"),
            recovery_vault_id: o.get_field("recoveryVaultId"),
            shutdown_recovery_group: o.get_field("shutdownRecoveryGroup"),
            source_recovery_fabric_id: o.get_field("sourceRecoveryFabricId"),
            target_recovery_fabric_id: o.get_field("targetRecoveryFabricId"),
        }
    }
}

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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod protection_container_mapping {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectionContainerMappingArgs {
        /// a `automatic_update` block defined as below.
        #[builder(into, default)]
        pub automatic_update: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::siterecovery::ProtectionContainerMappingAutomaticUpdate,
            >,
        >,
        /// The name of the protection container mapping. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of fabric that should contains the protection container to map. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_fabric_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Id of the policy to use for this mapping. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_replication_policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the source protection container to map. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_source_protection_container_name: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// Id of target protection container to map to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_target_protection_container_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProtectionContainerMappingResult {
        /// a `automatic_update` block defined as below.
        pub automatic_update: pulumi_gestalt_rust::Output<
            super::super::types::siterecovery::ProtectionContainerMappingAutomaticUpdate,
        >,
        /// The name of the protection container mapping. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Name of fabric that should contains the protection container to map. Changing this forces a new resource to be created.
        pub recovery_fabric_name: pulumi_gestalt_rust::Output<String>,
        /// Id of the policy to use for this mapping. Changing this forces a new resource to be created.
        pub recovery_replication_policy_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the source protection container to map. Changing this forces a new resource to be created.
        pub recovery_source_protection_container_name: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Id of target protection container to map to. Changing this forces a new resource to be created.
        pub recovery_target_protection_container_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProtectionContainerMappingArgs,
    ) -> ProtectionContainerMappingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automatic_update_binding = args.automatic_update.get_output(context);
        let name_binding = args.name.get_output(context);
        let recovery_fabric_name_binding = args.recovery_fabric_name.get_output(context);
        let recovery_replication_policy_id_binding = args
            .recovery_replication_policy_id
            .get_output(context);
        let recovery_source_protection_container_name_binding = args
            .recovery_source_protection_container_name
            .get_output(context);
        let recovery_target_protection_container_id_binding = args
            .recovery_target_protection_container_id
            .get_output(context);
        let recovery_vault_name_binding = args.recovery_vault_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:siterecovery/protectionContainerMapping:ProtectionContainerMapping"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticUpdate".into(),
                    value: automatic_update_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryFabricName".into(),
                    value: recovery_fabric_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryReplicationPolicyId".into(),
                    value: recovery_replication_policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoverySourceProtectionContainerName".into(),
                    value: recovery_source_protection_container_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryTargetProtectionContainerId".into(),
                    value: recovery_target_protection_container_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryVaultName".into(),
                    value: recovery_vault_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProtectionContainerMappingResult {
            automatic_update: o.get_field("automaticUpdate"),
            name: o.get_field("name"),
            recovery_fabric_name: o.get_field("recoveryFabricName"),
            recovery_replication_policy_id: o.get_field("recoveryReplicationPolicyId"),
            recovery_source_protection_container_name: o
                .get_field("recoverySourceProtectionContainerName"),
            recovery_target_protection_container_id: o
                .get_field("recoveryTargetProtectionContainerId"),
            recovery_vault_name: o.get_field("recoveryVaultName"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}

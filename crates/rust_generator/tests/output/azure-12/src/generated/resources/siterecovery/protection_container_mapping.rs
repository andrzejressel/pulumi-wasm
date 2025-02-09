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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProtectionContainerMappingArgs,
    ) -> ProtectionContainerMappingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let automatic_update_binding_1 = args.automatic_update.get_output(context);
        let automatic_update_binding = automatic_update_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let recovery_fabric_name_binding_1 = args
            .recovery_fabric_name
            .get_output(context);
        let recovery_fabric_name_binding = recovery_fabric_name_binding_1.get_inner();
        let recovery_replication_policy_id_binding_1 = args
            .recovery_replication_policy_id
            .get_output(context);
        let recovery_replication_policy_id_binding = recovery_replication_policy_id_binding_1
            .get_inner();
        let recovery_source_protection_container_name_binding_1 = args
            .recovery_source_protection_container_name
            .get_output(context);
        let recovery_source_protection_container_name_binding = recovery_source_protection_container_name_binding_1
            .get_inner();
        let recovery_target_protection_container_id_binding_1 = args
            .recovery_target_protection_container_id
            .get_output(context);
        let recovery_target_protection_container_id_binding = recovery_target_protection_container_id_binding_1
            .get_inner();
        let recovery_vault_name_binding_1 = args.recovery_vault_name.get_output(context);
        let recovery_vault_name_binding = recovery_vault_name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:siterecovery/protectionContainerMapping:ProtectionContainerMapping"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProtectionContainerMappingResult {
            automatic_update: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automaticUpdate"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            recovery_fabric_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryFabricName"),
            ),
            recovery_replication_policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryReplicationPolicyId"),
            ),
            recovery_source_protection_container_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoverySourceProtectionContainerName"),
            ),
            recovery_target_protection_container_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryTargetProtectionContainerId"),
            ),
            recovery_vault_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryVaultName"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}

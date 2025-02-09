/// Manages an Azure Site Recovery replication policy for VMWare within a Recovery Vault.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("East US")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleVMWareReplicationPolicy = vm_ware_replication_policy::create(
///         "exampleVMWareReplicationPolicy",
///         VmWareReplicationPolicyArgs::builder()
///             .application_consistent_snapshot_frequency_in_minutes(240)
///             .name("example-policy")
///             .recovery_point_retention_in_minutes(1440)
///             .recovery_vault_id("${exampleVault.id}")
///             .build_struct(),
///     );
///     let exampleVault = vault::create(
///         "exampleVault",
///         VaultArgs::builder()
///             .location("${example.location}")
///             .name("example-recovery-vault")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleVmwareReplicationPolicyAssociation = vmware_replication_policy_association::create(
///         "exampleVmwareReplicationPolicyAssociation",
///         VmwareReplicationPolicyAssociationArgs::builder()
///             .name("example-association")
///             .policy_id("${exampleVMWareReplicationPolicy.id}")
///             .recovery_vault_id("${exampleVault.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Site Recovery Replication Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/vmwareReplicationPolicyAssociation:VmwareReplicationPolicyAssociation mypolicy /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationFabrics/site-name/replicationProtectionContainers/container-name/replicationProtectionContainerMappings/mapping-name
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vmware_replication_policy_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VmwareReplicationPolicyAssociationArgs {
        /// The name of the replication policy association. Changing this forces a new association to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the VMWare replication policy which to be associated. Changing this forces a new association to be created.
        #[builder(into)]
        pub policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Recovery Service Vault to which the policy should be associated.
        /// Changing this forces a new association to be created.
        #[builder(into)]
        pub recovery_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VmwareReplicationPolicyAssociationResult {
        /// The name of the replication policy association. Changing this forces a new association to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VMWare replication policy which to be associated. Changing this forces a new association to be created.
        pub policy_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Recovery Service Vault to which the policy should be associated.
        /// Changing this forces a new association to be created.
        pub recovery_vault_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VmwareReplicationPolicyAssociationArgs,
    ) -> VmwareReplicationPolicyAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let policy_id_binding = args.policy_id.get_output(context);
        let recovery_vault_id_binding = args.recovery_vault_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:siterecovery/vmwareReplicationPolicyAssociation:VmwareReplicationPolicyAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyId".into(),
                    value: policy_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryVaultId".into(),
                    value: recovery_vault_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VmwareReplicationPolicyAssociationResult {
            name: o.get_field("name"),
            policy_id: o.get_field("policyId"),
            recovery_vault_id: o.get_field("recoveryVaultId"),
        }
    }
}

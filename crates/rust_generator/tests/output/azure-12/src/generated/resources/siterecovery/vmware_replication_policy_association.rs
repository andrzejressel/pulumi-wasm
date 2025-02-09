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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VmwareReplicationPolicyAssociationArgs,
    ) -> VmwareReplicationPolicyAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let policy_id_binding_1 = args.policy_id.get_output(context);
        let policy_id_binding = policy_id_binding_1.get_inner();
        let recovery_vault_id_binding_1 = args.recovery_vault_id.get_output(context);
        let recovery_vault_id_binding = recovery_vault_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:siterecovery/vmwareReplicationPolicyAssociation:VmwareReplicationPolicyAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryVaultId".into(),
                    value: &recovery_vault_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VmwareReplicationPolicyAssociationResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyId"),
            ),
            recovery_vault_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryVaultId"),
            ),
        }
    }
}

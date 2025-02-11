/// Manages an Azure Site Recovery replication policy for HyperV within a Recovery Vault.
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
///     let exampleHyperVReplicationPolicy = hyper_v_replication_policy::create(
///         "exampleHyperVReplicationPolicy",
///         HyperVReplicationPolicyArgs::builder()
///             .application_consistent_snapshot_frequency_in_hours(1)
///             .name("policy")
///             .recovery_point_retention_in_hours(2)
///             .recovery_vault_id("${exampleVault.id}")
///             .replication_interval_in_seconds(300)
///             .build_struct(),
///     );
///     let exampleHyperVReplicationPolicyAssociation = hyper_v_replication_policy_association::create(
///         "exampleHyperVReplicationPolicyAssociation",
///         HyperVReplicationPolicyAssociationArgs::builder()
///             .hyperv_site_id("${exampleHyperVSite.id}")
///             .name("example-association")
///             .policy_id("${exampleHyperVReplicationPolicy.id}")
///             .build_struct(),
///     );
///     let exampleHyperVSite = hyper_v_site::create(
///         "exampleHyperVSite",
///         HyperVSiteArgs::builder()
///             .name("example-site")
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
/// }
/// ```
///
/// ## Import
///
/// Site Recovery Replication Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/hyperVReplicationPolicyAssociation:HyperVReplicationPolicyAssociation azurerm_site_recovery_hyperv_replication_policy_association.mypolicy /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationFabrics/site-name/replicationProtectionContainers/container-name/replicationProtectionContainerMappings/mapping-name
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hyper_v_replication_policy_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HyperVReplicationPolicyAssociationArgs {
        /// The ID of the HyperV site to which the policy should be associated. Changing this forces a new association to be created.
        #[builder(into)]
        pub hyperv_site_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the replication policy association. Changing this forces a new association to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the HyperV replication policy which to be associated. Changing this forces a new association to be created.
        #[builder(into)]
        pub policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HyperVReplicationPolicyAssociationResult {
        /// The ID of the HyperV site to which the policy should be associated. Changing this forces a new association to be created.
        pub hyperv_site_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the replication policy association. Changing this forces a new association to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the HyperV replication policy which to be associated. Changing this forces a new association to be created.
        pub policy_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HyperVReplicationPolicyAssociationArgs,
    ) -> HyperVReplicationPolicyAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hyperv_site_id_binding = args.hyperv_site_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_id_binding = args.policy_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:siterecovery/hyperVReplicationPolicyAssociation:HyperVReplicationPolicyAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hypervSiteId".into(),
                    value: &hyperv_site_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HyperVReplicationPolicyAssociationResult {
            hyperv_site_id: o.get_field("hypervSiteId"),
            name: o.get_field("name"),
            policy_id: o.get_field("policyId"),
        }
    }
}

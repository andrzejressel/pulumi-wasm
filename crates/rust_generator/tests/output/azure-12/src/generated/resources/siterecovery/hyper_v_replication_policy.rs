/// Manages a Azure Site Recovery replication policy for HyperV within a Recovery Vault. Replication policies define the frequency at which recovery points are created and how long they are stored.
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
///     let policy = hyper_v_replication_policy::create(
///         "policy",
///         HyperVReplicationPolicyArgs::builder()
///             .application_consistent_snapshot_frequency_in_hours(1)
///             .name("policy")
///             .recovery_point_retention_in_hours(2)
///             .recovery_vault_id("${vault.id}")
///             .replication_interval_in_seconds(300)
///             .build_struct(),
///     );
///     let vault = vault::create(
///         "vault",
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
/// $ pulumi import azure:siterecovery/hyperVReplicationPolicy:HyperVReplicationPolicy mypolicy /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationPolicies/policy-name
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hyper_v_replication_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HyperVReplicationPolicyArgs {
        /// Specifies the frequency at which to create application consistent recovery points.
        #[builder(into)]
        pub application_consistent_snapshot_frequency_in_hours: pulumi_gestalt_rust::InputOrOutput<
            i32,
        >,
        /// The name of the replication policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The duration in hours for which the recovery points need to be stored.
        #[builder(into)]
        pub recovery_point_retention_in_hours: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The id of the vault that should be updated. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies how frequently data should be synchronized between source and target locations. Possible values are `30` and `300`.
        #[builder(into)]
        pub replication_interval_in_seconds: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct HyperVReplicationPolicyResult {
        /// Specifies the frequency at which to create application consistent recovery points.
        pub application_consistent_snapshot_frequency_in_hours: pulumi_gestalt_rust::Output<
            i32,
        >,
        /// The name of the replication policy. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The duration in hours for which the recovery points need to be stored.
        pub recovery_point_retention_in_hours: pulumi_gestalt_rust::Output<i32>,
        /// The id of the vault that should be updated. Changing this forces a new resource to be created.
        pub recovery_vault_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies how frequently data should be synchronized between source and target locations. Possible values are `30` and `300`.
        pub replication_interval_in_seconds: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HyperVReplicationPolicyArgs,
    ) -> HyperVReplicationPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_consistent_snapshot_frequency_in_hours_binding = args
            .application_consistent_snapshot_frequency_in_hours
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let recovery_point_retention_in_hours_binding = args
            .recovery_point_retention_in_hours
            .get_output(context);
        let recovery_vault_id_binding = args.recovery_vault_id.get_output(context);
        let replication_interval_in_seconds_binding = args
            .replication_interval_in_seconds
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:siterecovery/hyperVReplicationPolicy:HyperVReplicationPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationConsistentSnapshotFrequencyInHours".into(),
                    value: &application_consistent_snapshot_frequency_in_hours_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryPointRetentionInHours".into(),
                    value: &recovery_point_retention_in_hours_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryVaultId".into(),
                    value: &recovery_vault_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationIntervalInSeconds".into(),
                    value: &replication_interval_in_seconds_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HyperVReplicationPolicyResult {
            application_consistent_snapshot_frequency_in_hours: o
                .get_field("applicationConsistentSnapshotFrequencyInHours"),
            name: o.get_field("name"),
            recovery_point_retention_in_hours: o
                .get_field("recoveryPointRetentionInHours"),
            recovery_vault_id: o.get_field("recoveryVaultId"),
            replication_interval_in_seconds: o.get_field("replicationIntervalInSeconds"),
        }
    }
}

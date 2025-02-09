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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: HyperVReplicationPolicyArgs,
    ) -> HyperVReplicationPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_consistent_snapshot_frequency_in_hours_binding_1 = args
            .application_consistent_snapshot_frequency_in_hours
            .get_output(context);
        let application_consistent_snapshot_frequency_in_hours_binding = application_consistent_snapshot_frequency_in_hours_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let recovery_point_retention_in_hours_binding_1 = args
            .recovery_point_retention_in_hours
            .get_output(context);
        let recovery_point_retention_in_hours_binding = recovery_point_retention_in_hours_binding_1
            .get_inner();
        let recovery_vault_id_binding_1 = args.recovery_vault_id.get_output(context);
        let recovery_vault_id_binding = recovery_vault_id_binding_1.get_inner();
        let replication_interval_in_seconds_binding_1 = args
            .replication_interval_in_seconds
            .get_output(context);
        let replication_interval_in_seconds_binding = replication_interval_in_seconds_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:siterecovery/hyperVReplicationPolicy:HyperVReplicationPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationConsistentSnapshotFrequencyInHours".into(),
                    value: &application_consistent_snapshot_frequency_in_hours_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryPointRetentionInHours".into(),
                    value: &recovery_point_retention_in_hours_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryVaultId".into(),
                    value: &recovery_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "replicationIntervalInSeconds".into(),
                    value: &replication_interval_in_seconds_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HyperVReplicationPolicyResult {
            application_consistent_snapshot_frequency_in_hours: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationConsistentSnapshotFrequencyInHours"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            recovery_point_retention_in_hours: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryPointRetentionInHours"),
            ),
            recovery_vault_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryVaultId"),
            ),
            replication_interval_in_seconds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicationIntervalInSeconds"),
            ),
        }
    }
}

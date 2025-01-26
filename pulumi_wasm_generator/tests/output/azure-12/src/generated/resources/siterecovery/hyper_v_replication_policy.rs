/// Manages a Azure Site Recovery replication policy for HyperV within a Recovery Vault. Replication policies define the frequency at which recovery points are created and how long they are stored.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod hyper_v_replication_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HyperVReplicationPolicyArgs {
        /// Specifies the frequency at which to create application consistent recovery points.
        #[builder(into)]
        pub application_consistent_snapshot_frequency_in_hours: pulumi_wasm_rust::InputOrOutput<
            i32,
        >,
        /// The name of the replication policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The duration in hours for which the recovery points need to be stored.
        #[builder(into)]
        pub recovery_point_retention_in_hours: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The id of the vault that should be updated. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies how frequently data should be synchronized between source and target locations. Possible values are `30` and `300`.
        #[builder(into)]
        pub replication_interval_in_seconds: pulumi_wasm_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct HyperVReplicationPolicyResult {
        /// Specifies the frequency at which to create application consistent recovery points.
        pub application_consistent_snapshot_frequency_in_hours: pulumi_wasm_rust::Output<
            i32,
        >,
        /// The name of the replication policy. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The duration in hours for which the recovery points need to be stored.
        pub recovery_point_retention_in_hours: pulumi_wasm_rust::Output<i32>,
        /// The id of the vault that should be updated. Changing this forces a new resource to be created.
        pub recovery_vault_id: pulumi_wasm_rust::Output<String>,
        /// Specifies how frequently data should be synchronized between source and target locations. Possible values are `30` and `300`.
        pub replication_interval_in_seconds: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: HyperVReplicationPolicyArgs,
    ) -> HyperVReplicationPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_consistent_snapshot_frequency_in_hours_binding = args
            .application_consistent_snapshot_frequency_in_hours
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let recovery_point_retention_in_hours_binding = args
            .recovery_point_retention_in_hours
            .get_output(context)
            .get_inner();
        let recovery_vault_id_binding = args
            .recovery_vault_id
            .get_output(context)
            .get_inner();
        let replication_interval_in_seconds_binding = args
            .replication_interval_in_seconds
            .get_output(context)
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationConsistentSnapshotFrequencyInHours".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recoveryPointRetentionInHours".into(),
                },
                register_interface::ResultField {
                    name: "recoveryVaultId".into(),
                },
                register_interface::ResultField {
                    name: "replicationIntervalInSeconds".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HyperVReplicationPolicyResult {
            application_consistent_snapshot_frequency_in_hours: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationConsistentSnapshotFrequencyInHours").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recovery_point_retention_in_hours: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryPointRetentionInHours").unwrap(),
            ),
            recovery_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryVaultId").unwrap(),
            ),
            replication_interval_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationIntervalInSeconds").unwrap(),
            ),
        }
    }
}

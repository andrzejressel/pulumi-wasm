/// Manages a Azure Site Recovery replication policy within a recovery vault. Replication policies define the frequency at which recovery points are created and how long they are stored.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Site Recovery Replication Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:siterecovery/replicationPolicy:ReplicationPolicy mypolicy /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resource-group-name/providers/Microsoft.RecoveryServices/vaults/recovery-vault-name/replicationPolicies/policy-name
/// ```
///
pub mod replication_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationPolicyArgs {
        /// Specifies the frequency(in minutes) at which to create application consistent recovery points.
        ///
        /// > **Note:** The value of `application_consistent_snapshot_frequency_in_minutes` must be less than or equal to the value of `recovery_point_retention_in_minutes`.
        #[builder(into)]
        pub application_consistent_snapshot_frequency_in_minutes: pulumi_wasm_rust::InputOrOutput<
            i32,
        >,
        /// The name of the replication policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The duration in minutes for which the recovery points need to be stored.
        #[builder(into)]
        pub recovery_point_retention_in_minutes: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        #[builder(into)]
        pub recovery_vault_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReplicationPolicyResult {
        /// Specifies the frequency(in minutes) at which to create application consistent recovery points.
        ///
        /// > **Note:** The value of `application_consistent_snapshot_frequency_in_minutes` must be less than or equal to the value of `recovery_point_retention_in_minutes`.
        pub application_consistent_snapshot_frequency_in_minutes: pulumi_wasm_rust::Output<
            i32,
        >,
        /// The name of the replication policy. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The duration in minutes for which the recovery points need to be stored.
        pub recovery_point_retention_in_minutes: pulumi_wasm_rust::Output<i32>,
        /// The name of the vault that should be updated. Changing this forces a new resource to be created.
        pub recovery_vault_name: pulumi_wasm_rust::Output<String>,
        /// Name of the resource group where the vault that should be updated is located. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ReplicationPolicyArgs,
    ) -> ReplicationPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_consistent_snapshot_frequency_in_minutes_binding = args
            .application_consistent_snapshot_frequency_in_minutes
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let recovery_point_retention_in_minutes_binding = args
            .recovery_point_retention_in_minutes
            .get_output(context)
            .get_inner();
        let recovery_vault_name_binding = args
            .recovery_vault_name
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:siterecovery/replicationPolicy:ReplicationPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationConsistentSnapshotFrequencyInMinutes".into(),
                    value: &application_consistent_snapshot_frequency_in_minutes_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryPointRetentionInMinutes".into(),
                    value: &recovery_point_retention_in_minutes_binding,
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
        ReplicationPolicyResult {
            application_consistent_snapshot_frequency_in_minutes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationConsistentSnapshotFrequencyInMinutes"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            recovery_point_retention_in_minutes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("recoveryPointRetentionInMinutes"),
            ),
            recovery_vault_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("recoveryVaultName"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}

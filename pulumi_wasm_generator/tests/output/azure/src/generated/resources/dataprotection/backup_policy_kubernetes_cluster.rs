/// Manages a Backup Policy to back up Kubernetes Cluster.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleBackupVault:
///     type: azure:dataprotection:BackupVault
///     name: example
///     properties:
///       name: example-backup-vault
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       datastoreType: VaultStore
///       redundancy: LocallyRedundant
///   exampleBackupPolicyKubernetesCluster:
///     type: azure:dataprotection:BackupPolicyKubernetesCluster
///     name: example
///     properties:
///       name: example-backup-policy
///       resourceGroupName: ${example.name}
///       vaultName: ${exampleBackupVault.name}
///       backupRepeatingTimeIntervals:
///         - R/2021-05-23T02:30:00+00:00/P1W
///       timeZone: India Standard Time
///       defaultRetentionDuration: P4M
///       retentionRules:
///         - name: Daily
///           priority: 25
///           lifeCycles:
///             - duration: P84D
///               dataStoreType: OperationalStore
///           criteria:
///             absoluteCriteria: FirstOfDay
///       defaultRetentionRule:
///         lifeCycles:
///           - duration: P7D
///             dataStoreType: OperationalStore
/// ```
///
/// ## Import
///
/// Backup Policy Kubernetes Cluster's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupPolicyKubernetesCluster:BackupPolicyKubernetesCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1/backupPolicies/backupPolicy1
/// ```
///
pub mod backup_policy_kubernetes_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyKubernetesClusterArgs {
        /// Specifies a list of repeating time interval. It supports weekly back. It should follow `ISO 8601` repeating time interval. Changing this forces a new resource to be created.
        #[builder(into)]
        pub backup_repeating_time_intervals: pulumi_wasm_rust::Output<Vec<String>>,
        /// A `default_retention_rule` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub default_retention_rule: pulumi_wasm_rust::Output<
            super::super::types::dataprotection::BackupPolicyKubernetesClusterDefaultRetentionRule,
        >,
        /// The name which should be used for the Backup Policy Kubernetes Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Backup Policy Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub retention_rules: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyKubernetesClusterRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Backup Vault where the Backup Policy Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vault_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyKubernetesClusterResult {
        /// Specifies a list of repeating time interval. It supports weekly back. It should follow `ISO 8601` repeating time interval. Changing this forces a new resource to be created.
        pub backup_repeating_time_intervals: pulumi_wasm_rust::Output<Vec<String>>,
        /// A `default_retention_rule` block as defined below. Changing this forces a new resource to be created.
        pub default_retention_rule: pulumi_wasm_rust::Output<
            super::super::types::dataprotection::BackupPolicyKubernetesClusterDefaultRetentionRule,
        >,
        /// The name which should be used for the Backup Policy Kubernetes Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Backup Policy Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new resource to be created.
        pub retention_rules: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyKubernetesClusterRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new resource to be created.
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Backup Vault where the Backup Policy Kubernetes Cluster should exist. Changing this forces a new resource to be created.
        pub vault_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BackupPolicyKubernetesClusterArgs,
    ) -> BackupPolicyKubernetesClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_repeating_time_intervals_binding = args
            .backup_repeating_time_intervals
            .get_inner();
        let default_retention_rule_binding = args.default_retention_rule.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let retention_rules_binding = args.retention_rules.get_inner();
        let time_zone_binding = args.time_zone.get_inner();
        let vault_name_binding = args.vault_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dataprotection/backupPolicyKubernetesCluster:BackupPolicyKubernetesCluster"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupRepeatingTimeIntervals".into(),
                    value: &backup_repeating_time_intervals_binding,
                },
                register_interface::ObjectField {
                    name: "defaultRetentionRule".into(),
                    value: &default_retention_rule_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "retentionRules".into(),
                    value: &retention_rules_binding,
                },
                register_interface::ObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
                register_interface::ObjectField {
                    name: "vaultName".into(),
                    value: &vault_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backupRepeatingTimeIntervals".into(),
                },
                register_interface::ResultField {
                    name: "defaultRetentionRule".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "retentionRules".into(),
                },
                register_interface::ResultField {
                    name: "timeZone".into(),
                },
                register_interface::ResultField {
                    name: "vaultName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackupPolicyKubernetesClusterResult {
            backup_repeating_time_intervals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupRepeatingTimeIntervals").unwrap(),
            ),
            default_retention_rule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRetentionRule").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            retention_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionRules").unwrap(),
            ),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeZone").unwrap(),
            ),
            vault_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vaultName").unwrap(),
            ),
        }
    }
}
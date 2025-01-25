/// Manages a Backup Policy to back up PostgreSQL Flexible Server.
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
///       identity:
///         type: SystemAssigned
///   exampleBackupPolicyPostgresqlFlexibleServer:
///     type: azure:dataprotection:BackupPolicyPostgresqlFlexibleServer
///     name: example
///     properties:
///       name: example-backup-policy
///       vaultId: ${exampleBackupVault.id}
///       backupRepeatingTimeIntervals:
///         - R/2021-05-23T02:30:00+00:00/P1W
///       timeZone: India Standard Time
///       defaultRetentionRule:
///         lifeCycles:
///           - duration: P4M
///             dataStoreType: VaultStore
///       retentionRules:
///         - name: weekly
///           lifeCycles:
///             - duration: P6M
///               dataStoreType: VaultStore
///           priority: 20
///           criteria:
///             absoluteCriteria: FirstOfWeek
///         - name: thursday
///           lifeCycles:
///             - duration: P1W
///               dataStoreType: VaultStore
///           priority: 25
///           criteria:
///             daysOfWeeks:
///               - Thursday
///             scheduledBackupTimes:
///               - 2021-05-23T02:30:00Z
///         - name: monthly
///           lifeCycles:
///             - duration: P1D
///               dataStoreType: VaultStore
///           priority: 15
///           criteria:
///             weeksOfMonths:
///               - First
///               - Last
///             daysOfWeeks:
///               - Tuesday
///             scheduledBackupTimes:
///               - 2021-05-23T02:30:00Z
/// ```
///
/// ## Import
///
/// Backup Policy PostgreSQL Flexible Server's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupPolicyPostgresqlFlexibleServer:BackupPolicyPostgresqlFlexibleServer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1/backupPolicies/backupPolicy1
/// ```
///
pub mod backup_policy_postgresql_flexible_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyPostgresqlFlexibleServerArgs {
        /// Specifies a list of repeating time interval. It supports weekly back. It should follow `ISO 8601` repeating time interval format. Changing this forces a new resource to be created.
        #[builder(into)]
        pub backup_repeating_time_intervals: pulumi_wasm_rust::InputOrOutput<
            Vec<String>,
        >,
        /// A `default_retention_rule` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub default_retention_rule: pulumi_wasm_rust::InputOrOutput<
            super::super::types::dataprotection::BackupPolicyPostgresqlFlexibleServerDefaultRetentionRule,
        >,
        /// Specifies the name of the Backup Policy for the PostgreSQL Flexible Server. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub retention_rules: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyPostgresqlFlexibleServerRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub time_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Backup Vault where the Backup Policy PostgreSQL Flexible Server should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vault_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyPostgresqlFlexibleServerResult {
        /// Specifies a list of repeating time interval. It supports weekly back. It should follow `ISO 8601` repeating time interval format. Changing this forces a new resource to be created.
        pub backup_repeating_time_intervals: pulumi_wasm_rust::Output<Vec<String>>,
        /// A `default_retention_rule` block as defined below. Changing this forces a new resource to be created.
        pub default_retention_rule: pulumi_wasm_rust::Output<
            super::super::types::dataprotection::BackupPolicyPostgresqlFlexibleServerDefaultRetentionRule,
        >,
        /// Specifies the name of the Backup Policy for the PostgreSQL Flexible Server. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new resource to be created.
        pub retention_rules: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyPostgresqlFlexibleServerRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new resource to be created.
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Backup Vault where the Backup Policy PostgreSQL Flexible Server should exist. Changing this forces a new resource to be created.
        pub vault_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BackupPolicyPostgresqlFlexibleServerArgs,
    ) -> BackupPolicyPostgresqlFlexibleServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_repeating_time_intervals_binding = args
            .backup_repeating_time_intervals
            .get_output(context)
            .get_inner();
        let default_retention_rule_binding = args
            .default_retention_rule
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let retention_rules_binding = args
            .retention_rules
            .get_output(context)
            .get_inner();
        let time_zone_binding = args.time_zone.get_output(context).get_inner();
        let vault_id_binding = args.vault_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dataprotection/backupPolicyPostgresqlFlexibleServer:BackupPolicyPostgresqlFlexibleServer"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "retentionRules".into(),
                    value: &retention_rules_binding,
                },
                register_interface::ObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
                register_interface::ObjectField {
                    name: "vaultId".into(),
                    value: &vault_id_binding,
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
                    name: "retentionRules".into(),
                },
                register_interface::ResultField {
                    name: "timeZone".into(),
                },
                register_interface::ResultField {
                    name: "vaultId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackupPolicyPostgresqlFlexibleServerResult {
            backup_repeating_time_intervals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupRepeatingTimeIntervals").unwrap(),
            ),
            default_retention_rule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRetentionRule").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            retention_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionRules").unwrap(),
            ),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeZone").unwrap(),
            ),
            vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vaultId").unwrap(),
            ),
        }
    }
}

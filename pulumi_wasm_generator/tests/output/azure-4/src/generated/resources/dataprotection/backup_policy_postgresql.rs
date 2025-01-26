/// Manages a Backup Policy to back up PostgreSQL.
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
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleBackupPolicyPostgresql = backup_policy_postgresql::create(
///         "exampleBackupPolicyPostgresql",
///         BackupPolicyPostgresqlArgs::builder()
///             .backup_repeating_time_intervals(vec!["R/2021-05-23T02:30:00+00:00/P1W",])
///             .default_retention_duration("P4M")
///             .name("example-backup-policy")
///             .resource_group_name("${example.name}")
///             .retention_rules(
///                 vec![
///                     BackupPolicyPostgresqlRetentionRule::builder()
///                     .criteria(BackupPolicyPostgresqlRetentionRuleCriteria::builder()
///                     .absoluteCriteria("FirstOfWeek").build_struct()).duration("P6M")
///                     .name("weekly").priority(20).build_struct(),
///                     BackupPolicyPostgresqlRetentionRule::builder()
///                     .criteria(BackupPolicyPostgresqlRetentionRuleCriteria::builder()
///                     .daysOfWeeks(vec!["Thursday",])
///                     .scheduledBackupTimes(vec!["2021-05-23T02:30:00Z",]).build_struct())
///                     .duration("P1W").name("thursday").priority(25).build_struct(),
///                     BackupPolicyPostgresqlRetentionRule::builder()
///                     .criteria(BackupPolicyPostgresqlRetentionRuleCriteria::builder()
///                     .daysOfWeeks(vec!["Tuesday",])
///                     .scheduledBackupTimes(vec!["2021-05-23T02:30:00Z",])
///                     .weeksOfMonths(vec!["First", "Last",]).build_struct())
///                     .duration("P1D").name("monthly").priority(15).build_struct(),
///                 ],
///             )
///             .time_zone("India Standard Time")
///             .vault_name("${exampleBackupVault.name}")
///             .build_struct(),
///     );
///     let exampleBackupVault = backup_vault::create(
///         "exampleBackupVault",
///         BackupVaultArgs::builder()
///             .datastore_type("VaultStore")
///             .location("${example.location}")
///             .name("example-backup-vault")
///             .redundancy("LocallyRedundant")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Backup Policy PostgreSQL's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupPolicyPostgresql:BackupPolicyPostgresql example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1/backupPolicies/backupPolicy1
/// ```
///
pub mod backup_policy_postgresql {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyPostgresqlArgs {
        /// Specifies a list of repeating time interval. It supports weekly back. It should follow `ISO 8601` repeating time interval. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into)]
        pub backup_repeating_time_intervals: pulumi_wasm_rust::InputOrOutput<
            Vec<String>,
        >,
        /// The duration of default retention rule. It should follow `ISO 8601` duration format. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into)]
        pub default_retention_duration: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Backup Policy PostgreSQL. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Backup Policy PostgreSQL should exist. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into, default)]
        pub retention_rules: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyPostgresqlRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into, default)]
        pub time_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Backup Vault where the Backup Policy PostgreSQL should exist. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into)]
        pub vault_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyPostgresqlResult {
        /// Specifies a list of repeating time interval. It supports weekly back. It should follow `ISO 8601` repeating time interval. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub backup_repeating_time_intervals: pulumi_wasm_rust::Output<Vec<String>>,
        /// The duration of default retention rule. It should follow `ISO 8601` duration format. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub default_retention_duration: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Backup Policy PostgreSQL. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Backup Policy PostgreSQL should exist. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub retention_rules: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyPostgresqlRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Backup Vault where the Backup Policy PostgreSQL should exist. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub vault_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BackupPolicyPostgresqlArgs,
    ) -> BackupPolicyPostgresqlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_repeating_time_intervals_binding = args
            .backup_repeating_time_intervals
            .get_output(context)
            .get_inner();
        let default_retention_duration_binding = args
            .default_retention_duration
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let retention_rules_binding = args
            .retention_rules
            .get_output(context)
            .get_inner();
        let time_zone_binding = args.time_zone.get_output(context).get_inner();
        let vault_name_binding = args.vault_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dataprotection/backupPolicyPostgresql:BackupPolicyPostgresql"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupRepeatingTimeIntervals".into(),
                    value: &backup_repeating_time_intervals_binding,
                },
                register_interface::ObjectField {
                    name: "defaultRetentionDuration".into(),
                    value: &default_retention_duration_binding,
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
                    name: "defaultRetentionDuration".into(),
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackupPolicyPostgresqlResult {
            backup_repeating_time_intervals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupRepeatingTimeIntervals").unwrap(),
            ),
            default_retention_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRetentionDuration").unwrap(),
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

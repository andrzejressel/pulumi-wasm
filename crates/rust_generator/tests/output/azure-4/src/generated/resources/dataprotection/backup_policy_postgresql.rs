/// Manages a Backup Policy to back up PostgreSQL.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_policy_postgresql {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyPostgresqlArgs {
        /// Specifies a list of repeating time interval. It supports weekly back. It should follow `ISO 8601` repeating time interval. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into)]
        pub backup_repeating_time_intervals: pulumi_gestalt_rust::InputOrOutput<
            Vec<String>,
        >,
        /// The duration of default retention rule. It should follow `ISO 8601` duration format. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into)]
        pub default_retention_duration: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Backup Policy PostgreSQL. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Backup Policy PostgreSQL should exist. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into, default)]
        pub retention_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyPostgresqlRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into, default)]
        pub time_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Backup Vault where the Backup Policy PostgreSQL should exist. Changing this forces a new Backup Policy PostgreSQL to be created.
        #[builder(into)]
        pub vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyPostgresqlResult {
        /// Specifies a list of repeating time interval. It supports weekly back. It should follow `ISO 8601` repeating time interval. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub backup_repeating_time_intervals: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The duration of default retention rule. It should follow `ISO 8601` duration format. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub default_retention_duration: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Backup Policy PostgreSQL. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Backup Policy PostgreSQL should exist. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub retention_rules: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::dataprotection::BackupPolicyPostgresqlRetentionRule,
                >,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub time_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Backup Vault where the Backup Policy PostgreSQL should exist. Changing this forces a new Backup Policy PostgreSQL to be created.
        pub vault_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPolicyPostgresqlArgs,
    ) -> BackupPolicyPostgresqlResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_repeating_time_intervals_binding = args
            .backup_repeating_time_intervals
            .get_output(context);
        let default_retention_duration_binding = args
            .default_retention_duration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let retention_rules_binding = args.retention_rules.get_output(context);
        let time_zone_binding = args.time_zone.get_output(context);
        let vault_name_binding = args.vault_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:dataprotection/backupPolicyPostgresql:BackupPolicyPostgresql"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupRepeatingTimeIntervals".into(),
                    value: backup_repeating_time_intervals_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultRetentionDuration".into(),
                    value: default_retention_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionRules".into(),
                    value: retention_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeZone".into(),
                    value: time_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vaultName".into(),
                    value: vault_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupPolicyPostgresqlResult {
            backup_repeating_time_intervals: o.get_field("backupRepeatingTimeIntervals"),
            default_retention_duration: o.get_field("defaultRetentionDuration"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            retention_rules: o.get_field("retentionRules"),
            time_zone: o.get_field("timeZone"),
            vault_name: o.get_field("vaultName"),
        }
    }
}

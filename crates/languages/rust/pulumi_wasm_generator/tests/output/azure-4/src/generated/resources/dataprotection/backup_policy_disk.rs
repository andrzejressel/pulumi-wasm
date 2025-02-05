/// Manages a Backup Policy Disk.
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
///     let exampleBackupPolicyDisk = backup_policy_disk::create(
///         "exampleBackupPolicyDisk",
///         BackupPolicyDiskArgs::builder()
///             .backup_repeating_time_intervals(vec!["R/2021-05-19T06:33:16+00:00/PT4H",])
///             .default_retention_duration("P7D")
///             .name("example-backup-policy")
///             .retention_rules(
///                 vec![
///                     BackupPolicyDiskRetentionRule::builder()
///                     .criteria(BackupPolicyDiskRetentionRuleCriteria::builder()
///                     .absoluteCriteria("FirstOfDay").build_struct()).duration("P7D")
///                     .name("Daily").priority(25).build_struct(),
///                     BackupPolicyDiskRetentionRule::builder()
///                     .criteria(BackupPolicyDiskRetentionRuleCriteria::builder()
///                     .absoluteCriteria("FirstOfWeek").build_struct()).duration("P7D")
///                     .name("Weekly").priority(20).build_struct(),
///                 ],
///             )
///             .time_zone("W. Europe Standard Time")
///             .vault_id("${exampleBackupVault.id}")
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
/// Backup Policy Disks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dataprotection/backupPolicyDisk:BackupPolicyDisk example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataProtection/backupVaults/vault1/backupPolicies/backupPolicy1
/// ```
///
pub mod backup_policy_disk {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyDiskArgs {
        /// Specifies a list of repeating time interval. It should follow `ISO 8601` repeating time interval . Changing this forces a new Backup Policy Disk to be created.
        #[builder(into)]
        pub backup_repeating_time_intervals: pulumi_wasm_rust::InputOrOutput<
            Vec<String>,
        >,
        /// The duration of default retention rule. It should follow `ISO 8601` duration format. Changing this forces a new Backup Policy Disk to be created.
        #[builder(into)]
        pub default_retention_duration: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Backup Policy Disk. Changing this forces a new Backup Policy Disk to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new Backup Policy Disk to be created.
        #[builder(into, default)]
        pub retention_rules: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::dataprotection::BackupPolicyDiskRetentionRule>,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new Backup Policy Disk to be created.
        #[builder(into, default)]
        pub time_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Backup Vault within which the Backup Policy Disk should exist. Changing this forces a new Backup Policy Disk to be created.
        #[builder(into)]
        pub vault_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyDiskResult {
        /// Specifies a list of repeating time interval. It should follow `ISO 8601` repeating time interval . Changing this forces a new Backup Policy Disk to be created.
        pub backup_repeating_time_intervals: pulumi_wasm_rust::Output<Vec<String>>,
        /// The duration of default retention rule. It should follow `ISO 8601` duration format. Changing this forces a new Backup Policy Disk to be created.
        pub default_retention_duration: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Backup Policy Disk. Changing this forces a new Backup Policy Disk to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `retention_rule` blocks as defined below. Changing this forces a new Backup Policy Disk to be created.
        pub retention_rules: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::dataprotection::BackupPolicyDiskRetentionRule>,
            >,
        >,
        /// Specifies the Time Zone which should be used by the backup schedule. Changing this forces a new Backup Policy Disk to be created.
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Backup Vault within which the Backup Policy Disk should exist. Changing this forces a new Backup Policy Disk to be created.
        pub vault_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BackupPolicyDiskArgs,
    ) -> BackupPolicyDiskResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
        let retention_rules_binding = args
            .retention_rules
            .get_output(context)
            .get_inner();
        let time_zone_binding = args.time_zone.get_output(context).get_inner();
        let vault_id_binding = args.vault_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:dataprotection/backupPolicyDisk:BackupPolicyDisk".into(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackupPolicyDiskResult {
            backup_repeating_time_intervals: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupRepeatingTimeIntervals"),
            ),
            default_retention_duration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultRetentionDuration"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            retention_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retentionRules"),
            ),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeZone"),
            ),
            vault_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vaultId"),
            ),
        }
    }
}

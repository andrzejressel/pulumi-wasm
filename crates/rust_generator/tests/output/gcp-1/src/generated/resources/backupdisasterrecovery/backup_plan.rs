/// ## Example Usage
///
/// ### Backup Dr Backup Plan Simple
///
///
/// ```yaml
/// resources:
///   myBackupVault:
///     type: gcp:backupdisasterrecovery:BackupVault
///     name: my_backup_vault
///     properties:
///       location: us-central1
///       backupVaultId: backup-vault-simple-test
///       backupMinimumEnforcedRetentionDuration: 100000s
///   my-backup-plan-1:
///     type: gcp:backupdisasterrecovery:BackupPlan
///     properties:
///       location: us-central1
///       backupPlanId: backup-plan-simple-test
///       resourceType: compute.googleapis.com/Instance
///       backupVault: ${myBackupVault.id}
///       backupRules:
///         - ruleId: rule-1
///           backupRetentionDays: 5
///           standardSchedule:
///             recurrenceType: HOURLY
///             hourlyFrequency: 6
///             timeZone: UTC
///             backupWindow:
///               startHourOfDay: 0
///               endHourOfDay: 24
/// ```
///
/// ## Import
///
/// BackupPlan can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/backupPlans/{{backup_plan_id}}`
///
/// * `{{project}}/{{location}}/{{backup_plan_id}}`
///
/// * `{{location}}/{{backup_plan_id}}`
///
/// When using the `pulumi import` command, BackupPlan can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/backupPlan:BackupPlan default projects/{{project}}/locations/{{location}}/backupPlans/{{backup_plan_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/backupPlan:BackupPlan default {{project}}/{{location}}/{{backup_plan_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/backupPlan:BackupPlan default {{location}}/{{backup_plan_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPlanArgs {
        /// The ID of the backup plan
        #[builder(into)]
        pub backup_plan_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The backup rules for this `BackupPlan`. There must be at least one `BackupRule` message.
        /// Structure is documented below.
        #[builder(into)]
        pub backup_rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::backupdisasterrecovery::BackupPlanBackupRule>,
        >,
        /// Backup vault where the backups gets stored using this Backup plan.
        #[builder(into)]
        pub backup_vault: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description allows for additional details about 'BackupPlan' and its use cases to be provided.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location for the backup plan
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource type to which the `BackupPlan` will be applied. Examples include, "compute.googleapis.com/Instance" and "storage.googleapis.com/Bucket".
        #[builder(into)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPlanResult {
        /// The ID of the backup plan
        pub backup_plan_id: pulumi_gestalt_rust::Output<String>,
        /// The backup rules for this `BackupPlan`. There must be at least one `BackupRule` message.
        /// Structure is documented below.
        pub backup_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::backupdisasterrecovery::BackupPlanBackupRule>,
        >,
        /// Backup vault where the backups gets stored using this Backup plan.
        pub backup_vault: pulumi_gestalt_rust::Output<String>,
        /// The Google Cloud Platform Service Account to be used by the BackupVault for taking backups.
        pub backup_vault_service_account: pulumi_gestalt_rust::Output<String>,
        /// When the `BackupPlan` was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The description allows for additional details about 'BackupPlan' and its use cases to be provided.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location for the backup plan
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of backup plan resource created
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The resource type to which the `BackupPlan` will be applied. Examples include, "compute.googleapis.com/Instance" and "storage.googleapis.com/Bucket".
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// When the `BackupPlan` was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPlanArgs,
    ) -> BackupPlanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_plan_id_binding = args.backup_plan_id.get_output(context);
        let backup_rules_binding = args.backup_rules.get_output(context);
        let backup_vault_binding = args.backup_vault.get_output(context);
        let description_binding = args.description.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:backupdisasterrecovery/backupPlan:BackupPlan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupPlanId".into(),
                    value: backup_plan_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupRules".into(),
                    value: backup_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupVault".into(),
                    value: backup_vault_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: resource_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupPlanResult {
            backup_plan_id: o.get_field("backupPlanId"),
            backup_rules: o.get_field("backupRules"),
            backup_vault: o.get_field("backupVault"),
            backup_vault_service_account: o.get_field("backupVaultServiceAccount"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            resource_type: o.get_field("resourceType"),
            update_time: o.get_field("updateTime"),
        }
    }
}

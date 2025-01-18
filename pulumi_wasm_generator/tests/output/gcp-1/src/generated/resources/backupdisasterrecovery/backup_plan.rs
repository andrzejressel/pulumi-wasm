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
pub mod backup_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPlanArgs {
        /// The ID of the backup plan
        #[builder(into)]
        pub backup_plan_id: pulumi_wasm_rust::Output<String>,
        /// The backup rules for this `BackupPlan`. There must be at least one `BackupRule` message.
        /// Structure is documented below.
        #[builder(into)]
        pub backup_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::backupdisasterrecovery::BackupPlanBackupRule>,
        >,
        /// Backup vault where the backups gets stored using this Backup plan.
        #[builder(into)]
        pub backup_vault: pulumi_wasm_rust::Output<String>,
        /// The description allows for additional details about 'BackupPlan' and its use cases to be provided.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The location for the backup plan
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource type to which the `BackupPlan` will be applied. Examples include, "compute.googleapis.com/Instance" and "storage.googleapis.com/Bucket".
        #[builder(into)]
        pub resource_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPlanResult {
        /// The ID of the backup plan
        pub backup_plan_id: pulumi_wasm_rust::Output<String>,
        /// The backup rules for this `BackupPlan`. There must be at least one `BackupRule` message.
        /// Structure is documented below.
        pub backup_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::backupdisasterrecovery::BackupPlanBackupRule>,
        >,
        /// Backup vault where the backups gets stored using this Backup plan.
        pub backup_vault: pulumi_wasm_rust::Output<String>,
        /// The Google Cloud Platform Service Account to be used by the BackupVault for taking backups.
        pub backup_vault_service_account: pulumi_wasm_rust::Output<String>,
        /// When the `BackupPlan` was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The description allows for additional details about 'BackupPlan' and its use cases to be provided.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The location for the backup plan
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of backup plan resource created
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The resource type to which the `BackupPlan` will be applied. Examples include, "compute.googleapis.com/Instance" and "storage.googleapis.com/Bucket".
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// When the `BackupPlan` was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BackupPlanArgs) -> BackupPlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_plan_id_binding = args.backup_plan_id.get_inner();
        let backup_rules_binding = args.backup_rules.get_inner();
        let backup_vault_binding = args.backup_vault.get_inner();
        let description_binding = args.description.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let resource_type_binding = args.resource_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:backupdisasterrecovery/backupPlan:BackupPlan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupPlanId".into(),
                    value: &backup_plan_id_binding,
                },
                register_interface::ObjectField {
                    name: "backupRules".into(),
                    value: &backup_rules_binding,
                },
                register_interface::ObjectField {
                    name: "backupVault".into(),
                    value: &backup_vault_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backupPlanId".into(),
                },
                register_interface::ResultField {
                    name: "backupRules".into(),
                },
                register_interface::ResultField {
                    name: "backupVault".into(),
                },
                register_interface::ResultField {
                    name: "backupVaultServiceAccount".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackupPlanResult {
            backup_plan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupPlanId").unwrap(),
            ),
            backup_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupRules").unwrap(),
            ),
            backup_vault: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupVault").unwrap(),
            ),
            backup_vault_service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupVaultServiceAccount").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}

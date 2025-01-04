/// A backup policy is used to schedule backups at regular daily, weekly, or monthly intervals.
/// Backup policies allow you to attach a backup schedule to a volume.
/// The policy defines how many backups to retain at daily, weekly, or monthly intervals.
///
///
/// To get more information about BackupPolicy, see:
///
/// * [API documentation](https://cloud.google.com/netapp/volumes/docs/reference/rest/v1/projects.locations.backupPolicies)
/// * How-to Guides
///     * [Documentation](https://cloud.google.com/netapp/volumes/docs/protect-data/about-volume-backups#about_backup_policies)
///
/// ## Example Usage
///
/// ### Netapp Backup Policy Full
///
///
/// ```yaml
/// resources:
///   testBackupPolicyFull:
///     type: gcp:netapp:BackupPolicy
///     name: test_backup_policy_full
///     properties:
///       name: test-backup-policy-full
///       location: us-central1
///       dailyBackupLimit: 2
///       weeklyBackupLimit: 1
///       monthlyBackupLimit: 1
///       description: TF test backup schedule
///       enabled: true
///       labels:
///         foo: bar
/// ```
///
/// ## Import
///
/// BackupPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/backupPolicies/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, BackupPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:netapp/backupPolicy:BackupPolicy default projects/{{project}}/locations/{{location}}/backupPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/backupPolicy:BackupPolicy default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/backupPolicy:BackupPolicy default {{location}}/{{name}}
/// ```
///
pub mod backup_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyArgs {
        /// Number of daily backups to keep. Note that the minimum daily backup limit is 2.
        #[builder(into)]
        pub daily_backup_limit: pulumi_wasm_rust::Output<i32>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// If enabled, make backups automatically according to the schedules.
        /// This will be applied to all volumes that have this policy attached and enforced on volume level.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the region for the policy to apply to.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Number of monthly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1.
        #[builder(into)]
        pub monthly_backup_limit: pulumi_wasm_rust::Output<i32>,
        /// The name of the backup policy. Needs to be unique per location.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1.
        #[builder(into)]
        pub weekly_backup_limit: pulumi_wasm_rust::Output<i32>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyResult {
        /// The total number of volumes assigned by this backup policy.
        pub assigned_volume_count: pulumi_wasm_rust::Output<i32>,
        /// Create time of the backup policy. A timestamp in RFC3339 UTC "Zulu" format. Examples: "2023-06-22T09:13:01.617Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Number of daily backups to keep. Note that the minimum daily backup limit is 2.
        pub daily_backup_limit: pulumi_wasm_rust::Output<i32>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If enabled, make backups automatically according to the schedules.
        /// This will be applied to all volumes that have this policy attached and enforced on volume level.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the region for the policy to apply to.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Number of monthly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1.
        pub monthly_backup_limit: pulumi_wasm_rust::Output<i32>,
        /// The name of the backup policy. Needs to be unique per location.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The state of the backup policy.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1.
        pub weekly_backup_limit: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BackupPolicyArgs) -> BackupPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let daily_backup_limit_binding = args.daily_backup_limit.get_inner();
        let description_binding = args.description.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let monthly_backup_limit_binding = args.monthly_backup_limit.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let weekly_backup_limit_binding = args.weekly_backup_limit.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:netapp/backupPolicy:BackupPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dailyBackupLimit".into(),
                    value: &daily_backup_limit_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "monthlyBackupLimit".into(),
                    value: &monthly_backup_limit_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "weeklyBackupLimit".into(),
                    value: &weekly_backup_limit_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "assignedVolumeCount".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dailyBackupLimit".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "monthlyBackupLimit".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "weeklyBackupLimit".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackupPolicyResult {
            assigned_volume_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignedVolumeCount").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            daily_backup_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailyBackupLimit").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            monthly_backup_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monthlyBackupLimit").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            weekly_backup_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weeklyBackupLimit").unwrap(),
            ),
        }
    }
}

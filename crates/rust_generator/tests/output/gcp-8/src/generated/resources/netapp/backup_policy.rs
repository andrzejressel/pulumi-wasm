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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyArgs {
        /// Number of daily backups to keep. Note that the minimum daily backup limit is 2.
        #[builder(into)]
        pub daily_backup_limit: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If enabled, make backups automatically according to the schedules.
        /// This will be applied to all volumes that have this policy attached and enforced on volume level.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the region for the policy to apply to.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Number of monthly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1.
        #[builder(into)]
        pub monthly_backup_limit: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The name of the backup policy. Needs to be unique per location.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1.
        #[builder(into)]
        pub weekly_backup_limit: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyResult {
        /// The total number of volumes assigned by this backup policy.
        pub assigned_volume_count: pulumi_gestalt_rust::Output<i32>,
        /// Create time of the backup policy. A timestamp in RFC3339 UTC "Zulu" format. Examples: "2023-06-22T09:13:01.617Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Number of daily backups to keep. Note that the minimum daily backup limit is 2.
        pub daily_backup_limit: pulumi_gestalt_rust::Output<i32>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If enabled, make backups automatically according to the schedules.
        /// This will be applied to all volumes that have this policy attached and enforced on volume level.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the region for the policy to apply to.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Number of monthly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1.
        pub monthly_backup_limit: pulumi_gestalt_rust::Output<i32>,
        /// The name of the backup policy. Needs to be unique per location.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The state of the backup policy.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Number of weekly backups to keep. Note that the sum of daily, weekly and monthly backups should be greater than 1.
        pub weekly_backup_limit: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BackupPolicyArgs,
    ) -> BackupPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let daily_backup_limit_binding = args
            .daily_backup_limit
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let monthly_backup_limit_binding = args
            .monthly_backup_limit
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let weekly_backup_limit_binding = args
            .weekly_backup_limit
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:netapp/backupPolicy:BackupPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackupPolicyResult {
            assigned_volume_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assignedVolumeCount"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            daily_backup_limit: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dailyBackupLimit"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            monthly_backup_limit: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monthlyBackupLimit"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            weekly_backup_limit: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("weeklyBackupLimit"),
            ),
        }
    }
}

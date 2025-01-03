/// A backup schedule for a Cloud Firestore Database.
/// This resource is owned by the database it is backing up, and is deleted along with the database.
/// The actual backups are not though.
///
///
/// To get more information about BackupSchedule, see:
///
/// * [API documentation](https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.backupSchedules)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/firestore/docs/backups)
///
/// > **Warning:** This resource creates a Firestore Backup Schedule on a project that already has
/// a Firestore database.
/// This resource is owned by the database it is backing up, and is deleted along
/// with the database. The actual backups are not though.
///
/// ## Example Usage
///
/// ### Firestore Backup Schedule Daily
///
///
/// ```yaml
/// resources:
///   database:
///     type: gcp:firestore:Database
///     properties:
///       project: my-project-name
///       name: database-id
///       locationId: nam5
///       type: FIRESTORE_NATIVE
///       deleteProtectionState: DELETE_PROTECTION_ENABLED
///       deletionPolicy: DELETE
///   daily-backup:
///     type: gcp:firestore:BackupSchedule
///     properties:
///       project: my-project-name
///       database: ${database.name}
///       retention: 8467200s
///       dailyRecurrence: {}
/// ```
/// ### Firestore Backup Schedule Weekly
///
///
/// ```yaml
/// resources:
///   database:
///     type: gcp:firestore:Database
///     properties:
///       project: my-project-name
///       name: database-id
///       locationId: nam5
///       type: FIRESTORE_NATIVE
///       deleteProtectionState: DELETE_PROTECTION_ENABLED
///       deletionPolicy: DELETE
///   weekly-backup:
///     type: gcp:firestore:BackupSchedule
///     properties:
///       project: my-project-name
///       database: ${database.name}
///       retention: 8467200s
///       weeklyRecurrence:
///         day: SUNDAY
/// ```
///
/// ## Import
///
/// BackupSchedule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/databases/{{database}}/backupSchedules/{{name}}`
///
/// * `{{project}}/{{database}}/{{name}}`
///
/// * `{{database}}/{{name}}`
///
/// When using the `pulumi import` command, BackupSchedule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firestore/backupSchedule:BackupSchedule default projects/{{project}}/databases/{{database}}/backupSchedules/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firestore/backupSchedule:BackupSchedule default {{project}}/{{database}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firestore/backupSchedule:BackupSchedule default {{database}}/{{name}}
/// ```
///
pub mod backup_schedule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupScheduleArgs {
        /// For a schedule that runs daily.
        #[builder(into, default)]
        pub daily_recurrence: pulumi_wasm_rust::Output<
            Option<super::super::types::firestore::BackupScheduleDailyRecurrence>,
        >,
        /// The Firestore database id. Defaults to `"(default)"`.
        #[builder(into, default)]
        pub database: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// At what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        /// You can set this to a value up to 14 weeks.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub retention: pulumi_wasm_rust::Output<String>,
        /// For a schedule that runs weekly on a specific day.
        /// Structure is documented below.
        #[builder(into, default)]
        pub weekly_recurrence: pulumi_wasm_rust::Output<
            Option<super::super::types::firestore::BackupScheduleWeeklyRecurrence>,
        >,
    }
    #[allow(dead_code)]
    pub struct BackupScheduleResult {
        /// For a schedule that runs daily.
        pub daily_recurrence: pulumi_wasm_rust::Output<
            Option<super::super::types::firestore::BackupScheduleDailyRecurrence>,
        >,
        /// The Firestore database id. Defaults to `"(default)"`.
        pub database: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique backup schedule identifier across all locations and databases for the given project. Format:
        /// `projects/{{project}}/databases/{{database}}/backupSchedules/{{backupSchedule}}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// At what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        /// You can set this to a value up to 14 weeks.
        ///
        ///
        /// - - -
        pub retention: pulumi_wasm_rust::Output<String>,
        /// For a schedule that runs weekly on a specific day.
        /// Structure is documented below.
        pub weekly_recurrence: pulumi_wasm_rust::Output<
            Option<super::super::types::firestore::BackupScheduleWeeklyRecurrence>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BackupScheduleArgs) -> BackupScheduleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let daily_recurrence_binding = args.daily_recurrence.get_inner();
        let database_binding = args.database.get_inner();
        let project_binding = args.project.get_inner();
        let retention_binding = args.retention.get_inner();
        let weekly_recurrence_binding = args.weekly_recurrence.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firestore/backupSchedule:BackupSchedule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dailyRecurrence".into(),
                    value: &daily_recurrence_binding,
                },
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "retention".into(),
                    value: &retention_binding,
                },
                register_interface::ObjectField {
                    name: "weeklyRecurrence".into(),
                    value: &weekly_recurrence_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dailyRecurrence".into(),
                },
                register_interface::ResultField {
                    name: "database".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "retention".into(),
                },
                register_interface::ResultField {
                    name: "weeklyRecurrence".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BackupScheduleResult {
            daily_recurrence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailyRecurrence").unwrap(),
            ),
            database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("database").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            retention: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retention").unwrap(),
            ),
            weekly_recurrence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weeklyRecurrence").unwrap(),
            ),
        }
    }
}

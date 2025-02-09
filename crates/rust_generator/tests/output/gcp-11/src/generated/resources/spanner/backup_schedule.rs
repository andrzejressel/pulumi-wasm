/// A backup schedule for a Cloud Spanner Database.
/// This resource is owned by the database it is backing up, and is deleted along with the database.
/// The actual backups are not though.
///
///
/// To get more information about BackupSchedule, see:
///
/// * [API documentation](https://cloud.google.com/spanner/docs/reference/rest/v1/projects.instances.databases.backupSchedules)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/spanner/docs/backup)
///
/// > **Warning:** This resource creates a Spanner Backup Schedule on a project that already has
/// a Spanner database.
/// This resource is owned by the database it is backing up, and is deleted along
/// with the database. The actual backups are not though.
///
/// ## Example Usage
///
/// ### Spanner Backup Schedule Daily Full
///
///
/// ```yaml
/// resources:
///   main:
///     type: gcp:spanner:Instance
///     properties:
///       name: instance-id
///       config: regional-europe-west1
///       displayName: main-instance
///       numNodes: 1
///   database:
///     type: gcp:spanner:Database
///     properties:
///       instance: ${main.name}
///       name: database-id
///       versionRetentionPeriod: 3d
///       ddls:
///         - CREATE TABLE t1 (t1 INT64 NOT NULL,) PRIMARY KEY(t1)
///         - CREATE TABLE t2 (t2 INT64 NOT NULL,) PRIMARY KEY(t2)
///       deletionProtection: true
///   full-backup:
///     type: gcp:spanner:BackupSchedule
///     properties:
///       instance: ${main.name}
///       database: ${database.name}
///       name: backup-schedule-id
///       retentionDuration: 31620000s
///       spec:
///         cronSpec:
///           text: 0 12 * * *
///       fullBackupSpec: {}
/// ```
/// ### Spanner Backup Schedule Daily Incremental
///
///
/// ```yaml
/// resources:
///   main:
///     type: gcp:spanner:Instance
///     properties:
///       name: instance-id
///       config: regional-europe-west1
///       displayName: main-instance
///       numNodes: 1
///       edition: ENTERPRISE
///   database:
///     type: gcp:spanner:Database
///     properties:
///       instance: ${main.name}
///       name: database-id
///       versionRetentionPeriod: 3d
///       ddls:
///         - CREATE TABLE t1 (t1 INT64 NOT NULL,) PRIMARY KEY(t1)
///         - CREATE TABLE t2 (t2 INT64 NOT NULL,) PRIMARY KEY(t2)
///       deletionProtection: true
///   incremental-backup:
///     type: gcp:spanner:BackupSchedule
///     properties:
///       instance: ${main.name}
///       database: ${database.name}
///       name: backup-schedule-id
///       retentionDuration: 31620000s
///       spec:
///         cronSpec:
///           text: 0 12 * * *
///       incrementalBackupSpec: {}
/// ```
///
/// ## Import
///
/// BackupSchedule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/instances/{{instance}}/databases/{{database}}/backupSchedules/{{name}}`
///
/// * `{{project}}/{{instance}}/{{database}}/{{name}}`
///
/// * `{{instance}}/{{database}}/{{name}}`
///
/// When using the `pulumi import` command, BackupSchedule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:spanner/backupSchedule:BackupSchedule default projects/{{project}}/instances/{{instance}}/databases/{{database}}/backupSchedules/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:spanner/backupSchedule:BackupSchedule default {{project}}/{{instance}}/{{database}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:spanner/backupSchedule:BackupSchedule default {{instance}}/{{database}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupScheduleArgs {
        /// The database to create the backup schedule on.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub database: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The schedule creates only full backups..
        #[builder(into, default)]
        pub full_backup_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::spanner::BackupScheduleFullBackupSpec>,
        >,
        /// The schedule creates incremental backup chains.
        #[builder(into, default)]
        pub incremental_backup_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::spanner::BackupScheduleIncrementalBackupSpec>,
        >,
        /// The instance to create the database on.
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique identifier for the backup schedule, which cannot be changed after
        /// the backup schedule is created. Values are of the form [a-z][-a-z0-9]*[a-z0-9].
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// At what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'.
        /// You can set this to a value up to 366 days.
        #[builder(into)]
        pub retention_duration: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Defines specifications of the backup schedule.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::spanner::BackupScheduleSpec>,
        >,
    }
    #[allow(dead_code)]
    pub struct BackupScheduleResult {
        /// The database to create the backup schedule on.
        ///
        ///
        /// - - -
        pub database: pulumi_gestalt_rust::Output<String>,
        /// The schedule creates only full backups..
        pub full_backup_spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::spanner::BackupScheduleFullBackupSpec>,
        >,
        /// The schedule creates incremental backup chains.
        pub incremental_backup_spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::spanner::BackupScheduleIncrementalBackupSpec>,
        >,
        /// The instance to create the database on.
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for the backup schedule, which cannot be changed after
        /// the backup schedule is created. Values are of the form [a-z][-a-z0-9]*[a-z0-9].
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// At what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'.
        /// You can set this to a value up to 366 days.
        pub retention_duration: pulumi_gestalt_rust::Output<String>,
        /// Defines specifications of the backup schedule.
        /// Structure is documented below.
        pub spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::spanner::BackupScheduleSpec>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupScheduleArgs,
    ) -> BackupScheduleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let database_binding = args.database.get_output(context);
        let full_backup_spec_binding = args.full_backup_spec.get_output(context);
        let incremental_backup_spec_binding = args
            .incremental_backup_spec
            .get_output(context);
        let instance_binding = args.instance.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let retention_duration_binding = args.retention_duration.get_output(context);
        let spec_binding = args.spec.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:spanner/backupSchedule:BackupSchedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "database".into(),
                    value: database_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fullBackupSpec".into(),
                    value: full_backup_spec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "incrementalBackupSpec".into(),
                    value: incremental_backup_spec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: instance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionDuration".into(),
                    value: retention_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spec".into(),
                    value: spec_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupScheduleResult {
            database: o.get_field("database"),
            full_backup_spec: o.get_field("fullBackupSpec"),
            incremental_backup_spec: o.get_field("incrementalBackupSpec"),
            instance: o.get_field("instance"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            retention_duration: o.get_field("retentionDuration"),
            spec: o.get_field("spec"),
        }
    }
}

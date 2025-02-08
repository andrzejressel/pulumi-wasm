#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_backup_run {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackupRunArgs {
        /// The identifier for this backup run. Unique only for a specific Cloud SQL instance.
        /// If left empty and multiple backups exist for the instance, `most_recent` must be set to `true`.
        #[builder(into, default)]
        pub backup_id: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the instance the backup is taken from.
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Toggles use of the most recent backup run if multiple backups exist for a
        /// Cloud SQL instance.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The project to list instances for. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBackupRunResult {
        pub backup_id: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// Location of the backups.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The time the backup operation actually started in UTC timezone in RFC 3339 format, for
        /// example 2012-11-15T16:19:00.094Z.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// The status of this run. Refer to [API reference](https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1beta4/backupRuns#SqlBackupRunStatus) for possible status values.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetBackupRunArgs,
    ) -> GetBackupRunResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let backup_id_binding = args.backup_id.get_output(context).get_inner();
        let instance_binding = args.instance.get_output(context).get_inner();
        let most_recent_binding = args.most_recent.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:sql/getBackupRun:getBackupRun".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupId".into(),
                    value: &backup_id_binding,
                },
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBackupRunResult {
            backup_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instance"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            most_recent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mostRecent"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startTime"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}

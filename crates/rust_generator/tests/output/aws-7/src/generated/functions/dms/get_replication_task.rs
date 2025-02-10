#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_replication_task {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReplicationTaskArgs {
        /// The replication task identifier.
        ///
        /// - Must contain from 1 to 255 alphanumeric characters or hyphens.
        /// - First character must be a letter.
        /// - Cannot end with a hyphen.
        /// - Cannot contain two consecutive hyphens.
        #[builder(into)]
        pub replication_task_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReplicationTaskResult {
        /// (Conflicts with `cdc_start_time`) Indicates when you want a change data capture (CDC) operation to start. The value can be in date, checkpoint, or LSN/SCN format depending on the source engine. For more information, see [Determining a CDC native start point](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Task.CDC.html#CHAP_Task.CDC.StartPoint.Native).
        pub cdc_start_position: pulumi_gestalt_rust::Output<String>,
        /// (Conflicts with `cdc_start_position`) The Unix timestamp integer for the start of the Change Data Capture (CDC) operation.
        pub cdc_start_time: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The migration type. Can be one of `full-load | cdc | full-load-and-cdc`.
        pub migration_type: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the replication instance.
        pub replication_instance_arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the replication task.
        pub replication_task_arn: pulumi_gestalt_rust::Output<String>,
        pub replication_task_id: pulumi_gestalt_rust::Output<String>,
        /// An escaped JSON string that contains the task settings. For a complete list of task settings, see [Task Settings for AWS Database Migration Service Tasks](http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TaskSettings.html).
        pub replication_task_settings: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) string that uniquely identifies the source endpoint.
        pub source_endpoint_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether to run or stop the replication task.
        pub start_replication_task: pulumi_gestalt_rust::Output<bool>,
        /// Replication Task status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// An escaped JSON string that contains the table mappings. For information on table mapping see [Using Table Mapping with an AWS Database Migration Service Task to Select and Filter Data](http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TableMapping.html)
        pub table_mappings: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Amazon Resource Name (ARN) string that uniquely identifies the target endpoint.
        pub target_endpoint_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetReplicationTaskArgs,
    ) -> GetReplicationTaskResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let replication_task_id_binding = args.replication_task_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:dms/getReplicationTask:getReplicationTask".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicationTaskId".into(),
                    value: replication_task_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetReplicationTaskResult {
            cdc_start_position: o.get_field("cdcStartPosition"),
            cdc_start_time: o.get_field("cdcStartTime"),
            id: o.get_field("id"),
            migration_type: o.get_field("migrationType"),
            replication_instance_arn: o.get_field("replicationInstanceArn"),
            replication_task_arn: o.get_field("replicationTaskArn"),
            replication_task_id: o.get_field("replicationTaskId"),
            replication_task_settings: o.get_field("replicationTaskSettings"),
            source_endpoint_arn: o.get_field("sourceEndpointArn"),
            start_replication_task: o.get_field("startReplicationTask"),
            status: o.get_field("status"),
            table_mappings: o.get_field("tableMappings"),
            tags: o.get_field("tags"),
            target_endpoint_arn: o.get_field("targetEndpointArn"),
        }
    }
}

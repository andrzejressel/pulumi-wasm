/// Provides a DMS (Data Migration Service) replication task resource. DMS replication tasks can be created, updated, deleted, and imported.
///
/// > **NOTE:** Changing most arguments will stop the task if it is running. You can set `start_replication_task` to resume the task afterwards.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   # Create a new replication task
///   test:
///     type: aws:dms:ReplicationTask
///     properties:
///       cdcStartTime: 1993-05-21T05:50:00Z
///       migrationType: full-load
///       replicationInstanceArn: ${["test-dms-replication-instance-tf"].replicationInstanceArn}
///       replicationTaskId: test-dms-replication-task-tf
///       replicationTaskSettings: '...'
///       sourceEndpointArn: ${["test-dms-source-endpoint-tf"].endpointArn}
///       tableMappings: '{"rules":[{"rule-type":"selection","rule-id":"1","rule-name":"1","object-locator":{"schema-name":"%","table-name":"%"},"rule-action":"include"}]}'
///       tags:
///         Name: test
///       targetEndpointArn: ${["test-dms-target-endpoint-tf"].endpointArn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import replication tasks using the `replication_task_id`. For example:
///
/// ```sh
/// $ pulumi import aws:dms/replicationTask:ReplicationTask test test-dms-replication-task-tf
/// ```
pub mod replication_task {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationTaskArgs {
        /// Indicates when you want a change data capture (CDC) operation to start. The value can be a RFC3339 formatted date, a checkpoint, or a LSN/SCN format depending on the source engine. For more information see [Determining a CDC native start point](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Task.CDC.html#CHAP_Task.CDC.StartPoint.Native).
        #[builder(into, default)]
        pub cdc_start_position: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// RFC3339 formatted date string or UNIX timestamp for the start of the Change Data Capture (CDC) operation.
        #[builder(into, default)]
        pub cdc_start_time: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Migration type. Can be one of `full-load | cdc | full-load-and-cdc`.
        #[builder(into)]
        pub migration_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// ARN of the replication instance.
        #[builder(into)]
        pub replication_instance_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Replication task identifier which must contain from 1 to 255 alphanumeric characters or hyphens, first character must be a letter, cannot end with a hyphen, and cannot contain two consecutive hyphens.
        #[builder(into)]
        pub replication_task_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Escaped JSON string that contains the task settings. For a complete list of task settings, see [Task Settings for AWS Database Migration Service Tasks](http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TaskSettings.html). Note that `Logging.CloudWatchLogGroup` and `Logging.CloudWatchLogStream` are read only and should not be defined, even as `null`, in the configuration since AWS provides a value for these settings.
        #[builder(into, default)]
        pub replication_task_settings: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A friendly name for the resource identifier at the end of the EndpointArn response parameter that is returned in the created Endpoint object.
        #[builder(into, default)]
        pub resource_identifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ARN that uniquely identifies the source endpoint.
        #[builder(into)]
        pub source_endpoint_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether to run or stop the replication task.
        #[builder(into, default)]
        pub start_replication_task: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Escaped JSON string that contains the table mappings. For information on table mapping see [Using Table Mapping with an AWS Database Migration Service Task to Select and Filter Data](http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TableMapping.html)
        #[builder(into)]
        pub table_mappings: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN that uniquely identifies the target endpoint.
        #[builder(into)]
        pub target_endpoint_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReplicationTaskResult {
        /// Indicates when you want a change data capture (CDC) operation to start. The value can be a RFC3339 formatted date, a checkpoint, or a LSN/SCN format depending on the source engine. For more information see [Determining a CDC native start point](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Task.CDC.html#CHAP_Task.CDC.StartPoint.Native).
        pub cdc_start_position: pulumi_wasm_rust::Output<String>,
        /// RFC3339 formatted date string or UNIX timestamp for the start of the Change Data Capture (CDC) operation.
        pub cdc_start_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Migration type. Can be one of `full-load | cdc | full-load-and-cdc`.
        pub migration_type: pulumi_wasm_rust::Output<String>,
        /// ARN of the replication instance.
        pub replication_instance_arn: pulumi_wasm_rust::Output<String>,
        /// ARN for the replication task.
        pub replication_task_arn: pulumi_wasm_rust::Output<String>,
        /// Replication task identifier which must contain from 1 to 255 alphanumeric characters or hyphens, first character must be a letter, cannot end with a hyphen, and cannot contain two consecutive hyphens.
        pub replication_task_id: pulumi_wasm_rust::Output<String>,
        /// Escaped JSON string that contains the task settings. For a complete list of task settings, see [Task Settings for AWS Database Migration Service Tasks](http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TaskSettings.html). Note that `Logging.CloudWatchLogGroup` and `Logging.CloudWatchLogStream` are read only and should not be defined, even as `null`, in the configuration since AWS provides a value for these settings.
        pub replication_task_settings: pulumi_wasm_rust::Output<String>,
        /// A friendly name for the resource identifier at the end of the EndpointArn response parameter that is returned in the created Endpoint object.
        pub resource_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN that uniquely identifies the source endpoint.
        pub source_endpoint_arn: pulumi_wasm_rust::Output<String>,
        /// Whether to run or stop the replication task.
        pub start_replication_task: pulumi_wasm_rust::Output<Option<bool>>,
        /// Replication Task status.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Escaped JSON string that contains the table mappings. For information on table mapping see [Using Table Mapping with an AWS Database Migration Service Task to Select and Filter Data](http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TableMapping.html)
        pub table_mappings: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ARN that uniquely identifies the target endpoint.
        pub target_endpoint_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ReplicationTaskArgs,
    ) -> ReplicationTaskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cdc_start_position_binding = args
            .cdc_start_position
            .get_output(context)
            .get_inner();
        let cdc_start_time_binding = args.cdc_start_time.get_output(context).get_inner();
        let migration_type_binding = args.migration_type.get_output(context).get_inner();
        let replication_instance_arn_binding = args
            .replication_instance_arn
            .get_output(context)
            .get_inner();
        let replication_task_id_binding = args
            .replication_task_id
            .get_output(context)
            .get_inner();
        let replication_task_settings_binding = args
            .replication_task_settings
            .get_output(context)
            .get_inner();
        let resource_identifier_binding = args
            .resource_identifier
            .get_output(context)
            .get_inner();
        let source_endpoint_arn_binding = args
            .source_endpoint_arn
            .get_output(context)
            .get_inner();
        let start_replication_task_binding = args
            .start_replication_task
            .get_output(context)
            .get_inner();
        let table_mappings_binding = args.table_mappings.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_endpoint_arn_binding = args
            .target_endpoint_arn
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dms/replicationTask:ReplicationTask".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cdcStartPosition".into(),
                    value: &cdc_start_position_binding,
                },
                register_interface::ObjectField {
                    name: "cdcStartTime".into(),
                    value: &cdc_start_time_binding,
                },
                register_interface::ObjectField {
                    name: "migrationType".into(),
                    value: &migration_type_binding,
                },
                register_interface::ObjectField {
                    name: "replicationInstanceArn".into(),
                    value: &replication_instance_arn_binding,
                },
                register_interface::ObjectField {
                    name: "replicationTaskId".into(),
                    value: &replication_task_id_binding,
                },
                register_interface::ObjectField {
                    name: "replicationTaskSettings".into(),
                    value: &replication_task_settings_binding,
                },
                register_interface::ObjectField {
                    name: "resourceIdentifier".into(),
                    value: &resource_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "sourceEndpointArn".into(),
                    value: &source_endpoint_arn_binding,
                },
                register_interface::ObjectField {
                    name: "startReplicationTask".into(),
                    value: &start_replication_task_binding,
                },
                register_interface::ObjectField {
                    name: "tableMappings".into(),
                    value: &table_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetEndpointArn".into(),
                    value: &target_endpoint_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ReplicationTaskResult {
            cdc_start_position: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cdcStartPosition"),
            ),
            cdc_start_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cdcStartTime"),
            ),
            migration_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("migrationType"),
            ),
            replication_instance_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicationInstanceArn"),
            ),
            replication_task_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicationTaskArn"),
            ),
            replication_task_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicationTaskId"),
            ),
            replication_task_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicationTaskSettings"),
            ),
            resource_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceIdentifier"),
            ),
            source_endpoint_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceEndpointArn"),
            ),
            start_replication_task: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("startReplicationTask"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            table_mappings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tableMappings"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            target_endpoint_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetEndpointArn"),
            ),
        }
    }
}

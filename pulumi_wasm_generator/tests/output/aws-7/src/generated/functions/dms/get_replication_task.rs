pub mod get_replication_task {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
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
        pub replication_task_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReplicationTaskResult {
        /// (Conflicts with `cdc_start_time`) Indicates when you want a change data capture (CDC) operation to start. The value can be in date, checkpoint, or LSN/SCN format depending on the source engine. For more information, see [Determining a CDC native start point](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Task.CDC.html#CHAP_Task.CDC.StartPoint.Native).
        pub cdc_start_position: pulumi_wasm_rust::Output<String>,
        /// (Conflicts with `cdc_start_position`) The Unix timestamp integer for the start of the Change Data Capture (CDC) operation.
        pub cdc_start_time: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The migration type. Can be one of `full-load | cdc | full-load-and-cdc`.
        pub migration_type: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the replication instance.
        pub replication_instance_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the replication task.
        pub replication_task_arn: pulumi_wasm_rust::Output<String>,
        pub replication_task_id: pulumi_wasm_rust::Output<String>,
        /// An escaped JSON string that contains the task settings. For a complete list of task settings, see [Task Settings for AWS Database Migration Service Tasks](http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TaskSettings.html).
        pub replication_task_settings: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) string that uniquely identifies the source endpoint.
        pub source_endpoint_arn: pulumi_wasm_rust::Output<String>,
        /// Whether to run or stop the replication task.
        pub start_replication_task: pulumi_wasm_rust::Output<bool>,
        /// Replication Task status.
        pub status: pulumi_wasm_rust::Output<String>,
        /// An escaped JSON string that contains the table mappings. For information on table mapping see [Using Table Mapping with an AWS Database Migration Service Task to Select and Filter Data](http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TableMapping.html)
        pub table_mappings: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The Amazon Resource Name (ARN) string that uniquely identifies the target endpoint.
        pub target_endpoint_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetReplicationTaskArgs) -> GetReplicationTaskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let replication_task_id_binding = args.replication_task_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:dms/getReplicationTask:getReplicationTask".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "replicationTaskId".into(),
                    value: &replication_task_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cdcStartPosition".into(),
                },
                register_interface::ResultField {
                    name: "cdcStartTime".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "migrationType".into(),
                },
                register_interface::ResultField {
                    name: "replicationInstanceArn".into(),
                },
                register_interface::ResultField {
                    name: "replicationTaskArn".into(),
                },
                register_interface::ResultField {
                    name: "replicationTaskId".into(),
                },
                register_interface::ResultField {
                    name: "replicationTaskSettings".into(),
                },
                register_interface::ResultField {
                    name: "sourceEndpointArn".into(),
                },
                register_interface::ResultField {
                    name: "startReplicationTask".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tableMappings".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "targetEndpointArn".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetReplicationTaskResult {
            cdc_start_position: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdcStartPosition").unwrap(),
            ),
            cdc_start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdcStartTime").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            migration_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("migrationType").unwrap(),
            ),
            replication_instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationInstanceArn").unwrap(),
            ),
            replication_task_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationTaskArn").unwrap(),
            ),
            replication_task_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationTaskId").unwrap(),
            ),
            replication_task_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicationTaskSettings").unwrap(),
            ),
            source_endpoint_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceEndpointArn").unwrap(),
            ),
            start_replication_task: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startReplicationTask").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            table_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableMappings").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            target_endpoint_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetEndpointArn").unwrap(),
            ),
        }
    }
}

/// Manages an AWS DataSync Task, which represents a configuration for synchronization. Starting an execution of these DataSync Tasks (actually synchronizing files) is performed outside of this resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = task::create(
///         "example",
///         TaskArgs::builder()
///             .destination_location_arn("${destination.arn}")
///             .name("example")
///             .options(TaskOptions::builder().bytesPerSecond(-1).build_struct())
///             .source_location_arn("${source.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With Scheduling
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = task::create(
///         "example",
///         TaskArgs::builder()
///             .destination_location_arn("${destination.arn}")
///             .name("example")
///             .schedule(
///                 TaskSchedule::builder()
///                     .scheduleExpression("cron(0 12 ? * SUN,WED *)")
///                     .build_struct(),
///             )
///             .source_location_arn("${source.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With Filtering
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = task::create(
///         "example",
///         TaskArgs::builder()
///             .destination_location_arn("${destination.arn}")
///             .excludes(
///                 TaskExcludes::builder()
///                     .filterType("SIMPLE_PATTERN")
///                     .value("/folder1|/folder2")
///                     .build_struct(),
///             )
///             .includes(
///                 TaskIncludes::builder()
///                     .filterType("SIMPLE_PATTERN")
///                     .value("/folder1|/folder2")
///                     .build_struct(),
///             )
///             .name("example")
///             .source_location_arn("${source.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_task` using the DataSync Task Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:datasync/task:Task example arn:aws:datasync:us-east-1:123456789012:task/task-12345678901234567
/// ```
pub mod task {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TaskArgs {
        /// Amazon Resource Name (ARN) of the CloudWatch Log Group that is used to monitor and log events in the sync task.
        #[builder(into, default)]
        pub cloudwatch_log_group_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of destination DataSync Location.
        #[builder(into)]
        pub destination_location_arn: pulumi_wasm_rust::Output<String>,
        /// Filter rules that determines which files to exclude from a task.
        #[builder(into, default)]
        pub excludes: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::TaskExcludes>,
        >,
        /// Filter rules that determines which files to include in a task.
        #[builder(into, default)]
        pub includes: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::TaskIncludes>,
        >,
        /// Name of the DataSync Task.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block containing option that controls the default behavior when you start an execution of this DataSync Task. For each individual task execution, you can override these options by specifying an overriding configuration in those executions.
        #[builder(into, default)]
        pub options: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::TaskOptions>,
        >,
        /// Specifies a schedule used to periodically transfer files from a source to a destination location.
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::TaskSchedule>,
        >,
        /// Amazon Resource Name (ARN) of source DataSync Location.
        #[builder(into)]
        pub source_location_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Task. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block containing the configuration of a DataSync Task Report. See `task_report_config` below.
        #[builder(into, default)]
        pub task_report_config: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::TaskTaskReportConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct TaskResult {
        /// Amazon Resource Name (ARN) of the DataSync Task.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the CloudWatch Log Group that is used to monitor and log events in the sync task.
        pub cloudwatch_log_group_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of destination DataSync Location.
        pub destination_location_arn: pulumi_wasm_rust::Output<String>,
        /// Filter rules that determines which files to exclude from a task.
        pub excludes: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::TaskExcludes>,
        >,
        /// Filter rules that determines which files to include in a task.
        pub includes: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::TaskIncludes>,
        >,
        /// Name of the DataSync Task.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Configuration block containing option that controls the default behavior when you start an execution of this DataSync Task. For each individual task execution, you can override these options by specifying an overriding configuration in those executions.
        pub options: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::TaskOptions>,
        >,
        /// Specifies a schedule used to periodically transfer files from a source to a destination location.
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::TaskSchedule>,
        >,
        /// Amazon Resource Name (ARN) of source DataSync Location.
        pub source_location_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Task. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block containing the configuration of a DataSync Task Report. See `task_report_config` below.
        pub task_report_config: pulumi_wasm_rust::Output<
            Option<super::super::types::datasync::TaskTaskReportConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TaskArgs) -> TaskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloudwatch_log_group_arn_binding = args.cloudwatch_log_group_arn.get_inner();
        let destination_location_arn_binding = args.destination_location_arn.get_inner();
        let excludes_binding = args.excludes.get_inner();
        let includes_binding = args.includes.get_inner();
        let name_binding = args.name.get_inner();
        let options_binding = args.options.get_inner();
        let schedule_binding = args.schedule.get_inner();
        let source_location_arn_binding = args.source_location_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let task_report_config_binding = args.task_report_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/task:Task".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudwatchLogGroupArn".into(),
                    value: &cloudwatch_log_group_arn_binding,
                },
                register_interface::ObjectField {
                    name: "destinationLocationArn".into(),
                    value: &destination_location_arn_binding,
                },
                register_interface::ObjectField {
                    name: "excludes".into(),
                    value: &excludes_binding,
                },
                register_interface::ObjectField {
                    name: "includes".into(),
                    value: &includes_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "options".into(),
                    value: &options_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "sourceLocationArn".into(),
                    value: &source_location_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "taskReportConfig".into(),
                    value: &task_report_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cloudwatchLogGroupArn".into(),
                },
                register_interface::ResultField {
                    name: "destinationLocationArn".into(),
                },
                register_interface::ResultField {
                    name: "excludes".into(),
                },
                register_interface::ResultField {
                    name: "includes".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "options".into(),
                },
                register_interface::ResultField {
                    name: "schedule".into(),
                },
                register_interface::ResultField {
                    name: "sourceLocationArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "taskReportConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TaskResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cloudwatch_log_group_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudwatchLogGroupArn").unwrap(),
            ),
            destination_location_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationLocationArn").unwrap(),
            ),
            excludes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludes").unwrap(),
            ),
            includes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includes").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("options").unwrap(),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedule").unwrap(),
            ),
            source_location_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceLocationArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            task_report_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("taskReportConfig").unwrap(),
            ),
        }
    }
}

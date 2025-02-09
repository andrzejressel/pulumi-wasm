/// Manages an AWS DataSync Task, which represents a configuration for synchronization. Starting an execution of these DataSync Tasks (actually synchronizing files) is performed outside of this resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod task {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TaskArgs {
        /// Amazon Resource Name (ARN) of the CloudWatch Log Group that is used to monitor and log events in the sync task.
        #[builder(into, default)]
        pub cloudwatch_log_group_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of destination DataSync Location.
        #[builder(into)]
        pub destination_location_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Filter rules that determines which files to exclude from a task.
        #[builder(into, default)]
        pub excludes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datasync::TaskExcludes>,
        >,
        /// Filter rules that determines which files to include in a task.
        #[builder(into, default)]
        pub includes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datasync::TaskIncludes>,
        >,
        /// Name of the DataSync Task.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block containing option that controls the default behavior when you start an execution of this DataSync Task. For each individual task execution, you can override these options by specifying an overriding configuration in those executions.
        #[builder(into, default)]
        pub options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datasync::TaskOptions>,
        >,
        /// Specifies a schedule used to periodically transfer files from a source to a destination location.
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datasync::TaskSchedule>,
        >,
        /// Amazon Resource Name (ARN) of source DataSync Location.
        #[builder(into)]
        pub source_location_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Task. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block containing the configuration of a DataSync Task Report. See `task_report_config` below.
        #[builder(into, default)]
        pub task_report_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datasync::TaskTaskReportConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct TaskResult {
        /// Amazon Resource Name (ARN) of the DataSync Task.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the CloudWatch Log Group that is used to monitor and log events in the sync task.
        pub cloudwatch_log_group_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of destination DataSync Location.
        pub destination_location_arn: pulumi_gestalt_rust::Output<String>,
        /// Filter rules that determines which files to exclude from a task.
        pub excludes: pulumi_gestalt_rust::Output<
            Option<super::super::types::datasync::TaskExcludes>,
        >,
        /// Filter rules that determines which files to include in a task.
        pub includes: pulumi_gestalt_rust::Output<
            Option<super::super::types::datasync::TaskIncludes>,
        >,
        /// Name of the DataSync Task.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block containing option that controls the default behavior when you start an execution of this DataSync Task. For each individual task execution, you can override these options by specifying an overriding configuration in those executions.
        pub options: pulumi_gestalt_rust::Output<
            Option<super::super::types::datasync::TaskOptions>,
        >,
        /// Specifies a schedule used to periodically transfer files from a source to a destination location.
        pub schedule: pulumi_gestalt_rust::Output<
            Option<super::super::types::datasync::TaskSchedule>,
        >,
        /// Amazon Resource Name (ARN) of source DataSync Location.
        pub source_location_arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Task. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block containing the configuration of a DataSync Task Report. See `task_report_config` below.
        pub task_report_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::datasync::TaskTaskReportConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TaskArgs,
    ) -> TaskResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cloudwatch_log_group_arn_binding_1 = args
            .cloudwatch_log_group_arn
            .get_output(context);
        let cloudwatch_log_group_arn_binding = cloudwatch_log_group_arn_binding_1
            .get_inner();
        let destination_location_arn_binding_1 = args
            .destination_location_arn
            .get_output(context);
        let destination_location_arn_binding = destination_location_arn_binding_1
            .get_inner();
        let excludes_binding_1 = args.excludes.get_output(context);
        let excludes_binding = excludes_binding_1.get_inner();
        let includes_binding_1 = args.includes.get_output(context);
        let includes_binding = includes_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let options_binding_1 = args.options.get_output(context);
        let options_binding = options_binding_1.get_inner();
        let schedule_binding_1 = args.schedule.get_output(context);
        let schedule_binding = schedule_binding_1.get_inner();
        let source_location_arn_binding_1 = args.source_location_arn.get_output(context);
        let source_location_arn_binding = source_location_arn_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let task_report_config_binding_1 = args.task_report_config.get_output(context);
        let task_report_config_binding = task_report_config_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datasync/task:Task".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        TaskResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            cloudwatch_log_group_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloudwatchLogGroupArn"),
            ),
            destination_location_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationLocationArn"),
            ),
            excludes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("excludes"),
            ),
            includes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includes"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("options"),
            ),
            schedule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schedule"),
            ),
            source_location_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceLocationArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            task_report_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("taskReportConfig"),
            ),
        }
    }
}

/// Provides a SageMaker monitoring schedule resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = monitoring_schedule::create(
///         "test",
///         MonitoringScheduleArgs::builder()
///             .monitoring_schedule_config(
///                 MonitoringScheduleMonitoringScheduleConfig::builder()
///                     .monitoringJobDefinitionName(
///                         "${testAwsSagemakerDataQualityJobDefinition.name}",
///                     )
///                     .monitoringType("DataQuality")
///                     .build_struct(),
///             )
///             .name("my-monitoring-schedule")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import monitoring schedules using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/monitoringSchedule:MonitoringSchedule test_monitoring_schedule monitoring-schedule-foo
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod monitoring_schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitoringScheduleArgs {
        /// The configuration object that specifies the monitoring schedule and defines the monitoring job. Fields are documented below.
        #[builder(into)]
        pub monitoring_schedule_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::sagemaker::MonitoringScheduleMonitoringScheduleConfig,
        >,
        /// The name of the monitoring schedule. The name must be unique within an AWS Region within an AWS account. If omitted, the provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MonitoringScheduleResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this monitoring schedule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The configuration object that specifies the monitoring schedule and defines the monitoring job. Fields are documented below.
        pub monitoring_schedule_config: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::MonitoringScheduleMonitoringScheduleConfig,
        >,
        /// The name of the monitoring schedule. The name must be unique within an AWS Region within an AWS account. If omitted, the provider will assign a random, unique name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MonitoringScheduleArgs,
    ) -> MonitoringScheduleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let monitoring_schedule_config_binding = args
            .monitoring_schedule_config
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/monitoringSchedule:MonitoringSchedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "monitoringScheduleConfig".into(),
                    value: &monitoring_schedule_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MonitoringScheduleResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            monitoring_schedule_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoringScheduleConfig"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}

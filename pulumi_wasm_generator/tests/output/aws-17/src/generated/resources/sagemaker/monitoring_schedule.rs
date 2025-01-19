/// Provides a SageMaker monitoring schedule resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod monitoring_schedule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitoringScheduleArgs {
        /// The configuration object that specifies the monitoring schedule and defines the monitoring job. Fields are documented below.
        #[builder(into)]
        pub monitoring_schedule_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::MonitoringScheduleMonitoringScheduleConfig,
        >,
        /// The name of the monitoring schedule. The name must be unique within an AWS Region within an AWS account. If omitted, the provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MonitoringScheduleResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this monitoring schedule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The configuration object that specifies the monitoring schedule and defines the monitoring job. Fields are documented below.
        pub monitoring_schedule_config: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::MonitoringScheduleMonitoringScheduleConfig,
        >,
        /// The name of the monitoring schedule. The name must be unique within an AWS Region within an AWS account. If omitted, the provider will assign a random, unique name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MonitoringScheduleArgs) -> MonitoringScheduleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let monitoring_schedule_config_binding = args
            .monitoring_schedule_config
            .get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "monitoringScheduleConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MonitoringScheduleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            monitoring_schedule_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitoringScheduleConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}

/// Manages a Trigger Schedule inside a Azure Data Factory.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePipeline = pipeline::create(
///         "examplePipeline",
///         PipelineArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleTriggerSchedule = trigger_schedule::create(
///         "exampleTriggerSchedule",
///         TriggerScheduleArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .frequency("Day")
///             .interval(5)
///             .name("example")
///             .pipeline_name("${examplePipeline.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Schedule Trigger can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/triggerSchedule:TriggerSchedule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/triggers/example
/// ```
///
pub mod trigger_schedule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerScheduleArgs {
        /// Specifies if the Data Factory Schedule Trigger is activated. Defaults to `true`.
        #[builder(into, default)]
        pub activated: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of tags that can be used for describing the Data Factory Schedule Trigger.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The Schedule Trigger's description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The time the Schedule Trigger should end. The time will be represented in UTC.
        #[builder(into, default)]
        pub end_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The trigger frequency. Valid values include `Minute`, `Hour`, `Day`, `Week`, `Month`. Defaults to `Minute`.
        #[builder(into, default)]
        pub frequency: pulumi_wasm_rust::Output<Option<String>>,
        /// The interval for how often the trigger occurs. This defaults to `1`.
        #[builder(into, default)]
        pub interval: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Data Factory Schedule Trigger. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory Pipeline name that the trigger will act on.
        #[builder(into, default)]
        pub pipeline_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The pipeline parameters that the trigger will act upon.
        #[builder(into, default)]
        pub pipeline_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `pipeline` block as defined below.
        #[builder(into, default)]
        pub pipelines: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::datafactory::TriggerSchedulePipeline>>,
        >,
        /// A `schedule` block as defined below, which further specifies the recurrence schedule for the trigger. A schedule is capable of limiting or increasing the number of trigger executions specified by the `frequency` and `interval` properties.
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::TriggerScheduleSchedule>,
        >,
        /// The time the Schedule Trigger will start. This defaults to the current time. The time will be represented in UTC.
        #[builder(into, default)]
        pub start_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The timezone of the start/end time.
        #[builder(into, default)]
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TriggerScheduleResult {
        /// Specifies if the Data Factory Schedule Trigger is activated. Defaults to `true`.
        pub activated: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of tags that can be used for describing the Data Factory Schedule Trigger.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The Schedule Trigger's description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The time the Schedule Trigger should end. The time will be represented in UTC.
        pub end_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The trigger frequency. Valid values include `Minute`, `Hour`, `Day`, `Week`, `Month`. Defaults to `Minute`.
        pub frequency: pulumi_wasm_rust::Output<Option<String>>,
        /// The interval for how often the trigger occurs. This defaults to `1`.
        pub interval: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Data Factory Schedule Trigger. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Data Factory Pipeline name that the trigger will act on.
        pub pipeline_name: pulumi_wasm_rust::Output<String>,
        /// The pipeline parameters that the trigger will act upon.
        pub pipeline_parameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `pipeline` block as defined below.
        pub pipelines: pulumi_wasm_rust::Output<
            Vec<super::super::types::datafactory::TriggerSchedulePipeline>,
        >,
        /// A `schedule` block as defined below, which further specifies the recurrence schedule for the trigger. A schedule is capable of limiting or increasing the number of trigger executions specified by the `frequency` and `interval` properties.
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::TriggerScheduleSchedule>,
        >,
        /// The time the Schedule Trigger will start. This defaults to the current time. The time will be represented in UTC.
        pub start_time: pulumi_wasm_rust::Output<String>,
        /// The timezone of the start/end time.
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TriggerScheduleArgs) -> TriggerScheduleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let activated_binding = args.activated.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let end_time_binding = args.end_time.get_inner();
        let frequency_binding = args.frequency.get_inner();
        let interval_binding = args.interval.get_inner();
        let name_binding = args.name.get_inner();
        let pipeline_name_binding = args.pipeline_name.get_inner();
        let pipeline_parameters_binding = args.pipeline_parameters.get_inner();
        let pipelines_binding = args.pipelines.get_inner();
        let schedule_binding = args.schedule.get_inner();
        let start_time_binding = args.start_time.get_inner();
        let time_zone_binding = args.time_zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/triggerSchedule:TriggerSchedule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activated".into(),
                    value: &activated_binding,
                },
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "endTime".into(),
                    value: &end_time_binding,
                },
                register_interface::ObjectField {
                    name: "frequency".into(),
                    value: &frequency_binding,
                },
                register_interface::ObjectField {
                    name: "interval".into(),
                    value: &interval_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineName".into(),
                    value: &pipeline_name_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineParameters".into(),
                    value: &pipeline_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "pipelines".into(),
                    value: &pipelines_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding,
                },
                register_interface::ObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activated".into(),
                },
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "endTime".into(),
                },
                register_interface::ResultField {
                    name: "frequency".into(),
                },
                register_interface::ResultField {
                    name: "interval".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pipelineName".into(),
                },
                register_interface::ResultField {
                    name: "pipelineParameters".into(),
                },
                register_interface::ResultField {
                    name: "pipelines".into(),
                },
                register_interface::ResultField {
                    name: "schedule".into(),
                },
                register_interface::ResultField {
                    name: "startTime".into(),
                },
                register_interface::ResultField {
                    name: "timeZone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TriggerScheduleResult {
            activated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activated").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            end_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endTime").unwrap(),
            ),
            frequency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frequency").unwrap(),
            ),
            interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interval").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pipeline_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineName").unwrap(),
            ),
            pipeline_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineParameters").unwrap(),
            ),
            pipelines: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelines").unwrap(),
            ),
            schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedule").unwrap(),
            ),
            start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startTime").unwrap(),
            ),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeZone").unwrap(),
            ),
        }
    }
}

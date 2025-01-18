pub mod get_trigger_schedule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTriggerScheduleArgs {
        /// The ID of the Azure Data Factory to fetch trigger schedule from.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The name of the trigger schedule.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetTriggerScheduleResult {
        /// Specifies if the Data Factory Schedule Trigger is activated.
        pub activated: pulumi_wasm_rust::Output<bool>,
        /// List of tags that can be used for describing the Data Factory Schedule Trigger.
        pub annotations: pulumi_wasm_rust::Output<Vec<String>>,
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The Schedule Trigger's description.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The time the Schedule Trigger should end. The time will be represented in UTC.
        pub end_time: pulumi_wasm_rust::Output<String>,
        /// The trigger frequency.
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The interval for how often the trigger occurs.
        pub interval: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Data Factory Pipeline name that the trigger will act on.
        pub pipeline_name: pulumi_wasm_rust::Output<String>,
        /// A `schedule` block as described below, which further specifies the recurrence schedule for the trigger.
        pub schedules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::datafactory::GetTriggerScheduleSchedule>,
        >,
        /// The time the Schedule Trigger will start. The time will be represented in UTC.
        pub start_time: pulumi_wasm_rust::Output<String>,
        /// The timezone of the start/end time.
        pub time_zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTriggerScheduleArgs) -> GetTriggerScheduleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:datafactory/getTriggerSchedule:getTriggerSchedule".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "id".into(),
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
                    name: "schedules".into(),
                },
                register_interface::ResultField {
                    name: "startTime".into(),
                },
                register_interface::ResultField {
                    name: "timeZone".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTriggerScheduleResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interval").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pipeline_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineName").unwrap(),
            ),
            schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedules").unwrap(),
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

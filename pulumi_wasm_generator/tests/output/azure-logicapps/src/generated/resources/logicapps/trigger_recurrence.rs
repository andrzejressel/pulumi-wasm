/// Manages a Recurrence Trigger within a Logic App Workflow
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
///             .name("workflow-resources")
///             .build_struct(),
///     );
///     let exampleTriggerRecurrence = trigger_recurrence::create(
///         "exampleTriggerRecurrence",
///         TriggerRecurrenceArgs::builder()
///             .frequency("Day")
///             .interval(1)
///             .logic_app_id("${exampleWorkflow.id}")
///             .name("run-every-day")
///             .build_struct(),
///     );
///     let exampleWorkflow = workflow::create(
///         "exampleWorkflow",
///         WorkflowArgs::builder()
///             .location("${example.location}")
///             .name("workflow1")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Logic App Recurrence Triggers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/triggerRecurrence:TriggerRecurrence daily /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Logic/workflows/workflow1/triggers/daily
/// ```
///
pub mod trigger_recurrence {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerRecurrenceArgs {
        /// Specifies the Frequency at which this Trigger should be run. Possible values include `Month`, `Week`, `Day`, `Hour`, `Minute` and `Second`.
        #[builder(into)]
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// Specifies interval used for the Frequency, for example a value of `4` for `interval` and `hour` for `frequency` would run the Trigger every 4 hours.
        #[builder(into)]
        pub interval: pulumi_wasm_rust::Output<i32>,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        #[builder(into)]
        pub logic_app_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Recurrence Triggers to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Triggers within the Logic App Workflow.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `schedule` block as specified below.
        #[builder(into, default)]
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::logicapps::TriggerRecurrenceSchedule>,
        >,
        /// Specifies the start date and time for this trigger in RFC3339 format: `2000-01-02T03:04:05Z`.
        #[builder(into, default)]
        pub start_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the time zone for this trigger. Supported time zone options are listed [here](https://support.microsoft.com/en-us/help/973627/microsoft-time-zone-index-values)
        #[builder(into, default)]
        pub time_zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TriggerRecurrenceResult {
        /// Specifies the Frequency at which this Trigger should be run. Possible values include `Month`, `Week`, `Day`, `Hour`, `Minute` and `Second`.
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// Specifies interval used for the Frequency, for example a value of `4` for `interval` and `hour` for `frequency` would run the Trigger every 4 hours.
        pub interval: pulumi_wasm_rust::Output<i32>,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        pub logic_app_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Recurrence Triggers to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Triggers within the Logic App Workflow.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `schedule` block as specified below.
        pub schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::logicapps::TriggerRecurrenceSchedule>,
        >,
        /// Specifies the start date and time for this trigger in RFC3339 format: `2000-01-02T03:04:05Z`.
        pub start_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the time zone for this trigger. Supported time zone options are listed [here](https://support.microsoft.com/en-us/help/973627/microsoft-time-zone-index-values)
        pub time_zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TriggerRecurrenceArgs) -> TriggerRecurrenceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let frequency_binding = args.frequency.get_inner();
        let interval_binding = args.interval.get_inner();
        let logic_app_id_binding = args.logic_app_id.get_inner();
        let name_binding = args.name.get_inner();
        let schedule_binding = args.schedule.get_inner();
        let start_time_binding = args.start_time.get_inner();
        let time_zone_binding = args.time_zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/triggerRecurrence:TriggerRecurrence".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "frequency".into(),
                    value: &frequency_binding,
                },
                register_interface::ObjectField {
                    name: "interval".into(),
                    value: &interval_binding,
                },
                register_interface::ObjectField {
                    name: "logicAppId".into(),
                    value: &logic_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "frequency".into(),
                },
                register_interface::ResultField {
                    name: "interval".into(),
                },
                register_interface::ResultField {
                    name: "logicAppId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        TriggerRecurrenceResult {
            frequency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frequency").unwrap(),
            ),
            interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interval").unwrap(),
            ),
            logic_app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logicAppId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
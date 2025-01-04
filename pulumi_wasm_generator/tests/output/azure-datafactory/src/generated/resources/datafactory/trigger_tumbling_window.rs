/// Manages a Tumbling Window Trigger inside an Azure Data Factory.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   examplePipeline:
///     type: azure:datafactory:Pipeline
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///   exampleTriggerTumblingWindow:
///     type: azure:datafactory:TriggerTumblingWindow
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       startTime: 2022-09-21T00:00:00Z
///       endTime: 2022-09-21T08:00:00Z
///       frequency: Minute
///       interval: 15
///       delay: 16:00:00
///       annotations:
///         - example1
///         - example2
///         - example3
///       description: example description
///       retry:
///         count: 1
///         interval: 30
///       pipeline:
///         name: ${examplePipeline.name}
///         parameters:
///           Env: Prod
///       triggerDependencies:
///         - size: 24:00:00
///           offset: -24:00:00
///       additionalProperties:
///         foo: value1
///         bar: value2
/// ```
///
/// ## Import
///
/// Data Factory Tumbling Window Trigger can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/triggerTumblingWindow:TriggerTumblingWindow example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/triggers/example
/// ```
///
pub mod trigger_tumbling_window {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerTumblingWindowArgs {
        /// Specifies if the Data Factory Tumbling Window Trigger is activated. Defaults to `true`.
        #[builder(into, default)]
        pub activated: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Tumbling Window Trigger.
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Tumbling Window Trigger.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// Specifies how long the trigger waits before triggering new run. formatted as an `D.HH:MM:SS`.
        #[builder(into, default)]
        pub delay: pulumi_wasm_rust::Output<Option<String>>,
        /// The description for the Data Factory Tumbling Window Trigger.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the end time of Tumbling Window, formatted as an RFC3339 string.
        #[builder(into, default)]
        pub end_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the frequency of Tumbling Window. Possible values are `Hour`, `Minute` and `Month`. Changing this forces a new resource.
        #[builder(into)]
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// Specifies the interval of Tumbling Window. Changing this forces a new resource.
        #[builder(into)]
        pub interval: pulumi_wasm_rust::Output<i32>,
        /// The max number for simultaneous trigger run fired by Tumbling Window. Possible values are between `1` and `50`. Defaults to `50`.
        #[builder(into, default)]
        pub max_concurrency: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Data Factory Tumbling Window Trigger. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `pipeline` block as defined below.
        #[builder(into)]
        pub pipeline: pulumi_wasm_rust::Output<
            super::super::types::datafactory::TriggerTumblingWindowPipeline,
        >,
        /// A `retry` block as defined below.
        #[builder(into, default)]
        pub retry: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::TriggerTumblingWindowRetry>,
        >,
        /// Specifies the start time of Tumbling Window, formatted as an RFC3339 string. Changing this forces a new resource.
        #[builder(into)]
        pub start_time: pulumi_wasm_rust::Output<String>,
        /// One or more `trigger_dependency` block as defined below.
        #[builder(into, default)]
        pub trigger_dependencies: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::datafactory::TriggerTumblingWindowTriggerDependency,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct TriggerTumblingWindowResult {
        /// Specifies if the Data Factory Tumbling Window Trigger is activated. Defaults to `true`.
        pub activated: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Tumbling Window Trigger.
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Tumbling Window Trigger.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// Specifies how long the trigger waits before triggering new run. formatted as an `D.HH:MM:SS`.
        pub delay: pulumi_wasm_rust::Output<Option<String>>,
        /// The description for the Data Factory Tumbling Window Trigger.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the end time of Tumbling Window, formatted as an RFC3339 string.
        pub end_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the frequency of Tumbling Window. Possible values are `Hour`, `Minute` and `Month`. Changing this forces a new resource.
        pub frequency: pulumi_wasm_rust::Output<String>,
        /// Specifies the interval of Tumbling Window. Changing this forces a new resource.
        pub interval: pulumi_wasm_rust::Output<i32>,
        /// The max number for simultaneous trigger run fired by Tumbling Window. Possible values are between `1` and `50`. Defaults to `50`.
        pub max_concurrency: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Data Factory Tumbling Window Trigger. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `pipeline` block as defined below.
        pub pipeline: pulumi_wasm_rust::Output<
            super::super::types::datafactory::TriggerTumblingWindowPipeline,
        >,
        /// A `retry` block as defined below.
        pub retry: pulumi_wasm_rust::Output<
            Option<super::super::types::datafactory::TriggerTumblingWindowRetry>,
        >,
        /// Specifies the start time of Tumbling Window, formatted as an RFC3339 string. Changing this forces a new resource.
        pub start_time: pulumi_wasm_rust::Output<String>,
        /// One or more `trigger_dependency` block as defined below.
        pub trigger_dependencies: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::datafactory::TriggerTumblingWindowTriggerDependency,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TriggerTumblingWindowArgs,
    ) -> TriggerTumblingWindowResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let activated_binding = args.activated.get_inner();
        let additional_properties_binding = args.additional_properties.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let delay_binding = args.delay.get_inner();
        let description_binding = args.description.get_inner();
        let end_time_binding = args.end_time.get_inner();
        let frequency_binding = args.frequency.get_inner();
        let interval_binding = args.interval.get_inner();
        let max_concurrency_binding = args.max_concurrency.get_inner();
        let name_binding = args.name.get_inner();
        let pipeline_binding = args.pipeline.get_inner();
        let retry_binding = args.retry.get_inner();
        let start_time_binding = args.start_time.get_inner();
        let trigger_dependencies_binding = args.trigger_dependencies.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/triggerTumblingWindow:TriggerTumblingWindow"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activated".into(),
                    value: &activated_binding,
                },
                register_interface::ObjectField {
                    name: "additionalProperties".into(),
                    value: &additional_properties_binding,
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
                    name: "delay".into(),
                    value: &delay_binding,
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
                    name: "maxConcurrency".into(),
                    value: &max_concurrency_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pipeline".into(),
                    value: &pipeline_binding,
                },
                register_interface::ObjectField {
                    name: "retry".into(),
                    value: &retry_binding,
                },
                register_interface::ObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding,
                },
                register_interface::ObjectField {
                    name: "triggerDependencies".into(),
                    value: &trigger_dependencies_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activated".into(),
                },
                register_interface::ResultField {
                    name: "additionalProperties".into(),
                },
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "delay".into(),
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
                    name: "maxConcurrency".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pipeline".into(),
                },
                register_interface::ResultField {
                    name: "retry".into(),
                },
                register_interface::ResultField {
                    name: "startTime".into(),
                },
                register_interface::ResultField {
                    name: "triggerDependencies".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TriggerTumblingWindowResult {
            activated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activated").unwrap(),
            ),
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalProperties").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            delay: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delay").unwrap(),
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
            max_concurrency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxConcurrency").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pipeline: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipeline").unwrap(),
            ),
            retry: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retry").unwrap(),
            ),
            start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startTime").unwrap(),
            ),
            trigger_dependencies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggerDependencies").unwrap(),
            ),
        }
    }
}

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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trigger_tumbling_window {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerTumblingWindowArgs {
        /// Specifies if the Data Factory Tumbling Window Trigger is activated. Defaults to `true`.
        #[builder(into, default)]
        pub activated: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Tumbling Window Trigger.
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Tumbling Window Trigger.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies how long the trigger waits before triggering new run. formatted as an `D.HH:MM:SS`.
        #[builder(into, default)]
        pub delay: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description for the Data Factory Tumbling Window Trigger.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the end time of Tumbling Window, formatted as an RFC3339 string.
        #[builder(into, default)]
        pub end_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the frequency of Tumbling Window. Possible values are `Hour`, `Minute` and `Month`. Changing this forces a new resource.
        #[builder(into)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the interval of Tumbling Window. Changing this forces a new resource.
        #[builder(into)]
        pub interval: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The max number for simultaneous trigger run fired by Tumbling Window. Possible values are between `1` and `50`. Defaults to `50`.
        #[builder(into, default)]
        pub max_concurrency: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of the Data Factory Tumbling Window Trigger. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `pipeline` block as defined below.
        #[builder(into)]
        pub pipeline: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::datafactory::TriggerTumblingWindowPipeline,
        >,
        /// A `retry` block as defined below.
        #[builder(into, default)]
        pub retry: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::TriggerTumblingWindowRetry>,
        >,
        /// Specifies the start time of Tumbling Window, formatted as an RFC3339 string. Changing this forces a new resource.
        #[builder(into)]
        pub start_time: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `trigger_dependency` block as defined below.
        #[builder(into, default)]
        pub trigger_dependencies: pulumi_gestalt_rust::InputOrOutput<
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
        pub activated: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Tumbling Window Trigger.
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Tumbling Window Trigger.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies how long the trigger waits before triggering new run. formatted as an `D.HH:MM:SS`.
        pub delay: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description for the Data Factory Tumbling Window Trigger.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the end time of Tumbling Window, formatted as an RFC3339 string.
        pub end_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the frequency of Tumbling Window. Possible values are `Hour`, `Minute` and `Month`. Changing this forces a new resource.
        pub frequency: pulumi_gestalt_rust::Output<String>,
        /// Specifies the interval of Tumbling Window. Changing this forces a new resource.
        pub interval: pulumi_gestalt_rust::Output<i32>,
        /// The max number for simultaneous trigger run fired by Tumbling Window. Possible values are between `1` and `50`. Defaults to `50`.
        pub max_concurrency: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the name of the Data Factory Tumbling Window Trigger. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `pipeline` block as defined below.
        pub pipeline: pulumi_gestalt_rust::Output<
            super::super::types::datafactory::TriggerTumblingWindowPipeline,
        >,
        /// A `retry` block as defined below.
        pub retry: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::TriggerTumblingWindowRetry>,
        >,
        /// Specifies the start time of Tumbling Window, formatted as an RFC3339 string. Changing this forces a new resource.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// One or more `trigger_dependency` block as defined below.
        pub trigger_dependencies: pulumi_gestalt_rust::Output<
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TriggerTumblingWindowArgs,
    ) -> TriggerTumblingWindowResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let activated_binding_1 = args.activated.get_output(context);
        let activated_binding = activated_binding_1.get_inner();
        let additional_properties_binding_1 = args
            .additional_properties
            .get_output(context);
        let additional_properties_binding = additional_properties_binding_1.get_inner();
        let annotations_binding_1 = args.annotations.get_output(context);
        let annotations_binding = annotations_binding_1.get_inner();
        let data_factory_id_binding_1 = args.data_factory_id.get_output(context);
        let data_factory_id_binding = data_factory_id_binding_1.get_inner();
        let delay_binding_1 = args.delay.get_output(context);
        let delay_binding = delay_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let end_time_binding_1 = args.end_time.get_output(context);
        let end_time_binding = end_time_binding_1.get_inner();
        let frequency_binding_1 = args.frequency.get_output(context);
        let frequency_binding = frequency_binding_1.get_inner();
        let interval_binding_1 = args.interval.get_output(context);
        let interval_binding = interval_binding_1.get_inner();
        let max_concurrency_binding_1 = args.max_concurrency.get_output(context);
        let max_concurrency_binding = max_concurrency_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let pipeline_binding_1 = args.pipeline.get_output(context);
        let pipeline_binding = pipeline_binding_1.get_inner();
        let retry_binding_1 = args.retry.get_output(context);
        let retry_binding = retry_binding_1.get_inner();
        let start_time_binding_1 = args.start_time.get_output(context);
        let start_time_binding = start_time_binding_1.get_inner();
        let trigger_dependencies_binding_1 = args
            .trigger_dependencies
            .get_output(context);
        let trigger_dependencies_binding = trigger_dependencies_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/triggerTumblingWindow:TriggerTumblingWindow"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        TriggerTumblingWindowResult {
            activated: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("activated"),
            ),
            additional_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalProperties"),
            ),
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            data_factory_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataFactoryId"),
            ),
            delay: pulumi_gestalt_rust::__private::into_domain(o.extract_field("delay")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            end_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endTime"),
            ),
            frequency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frequency"),
            ),
            interval: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("interval"),
            ),
            max_concurrency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxConcurrency"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            pipeline: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pipeline"),
            ),
            retry: pulumi_gestalt_rust::__private::into_domain(o.extract_field("retry")),
            start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startTime"),
            ),
            trigger_dependencies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("triggerDependencies"),
            ),
        }
    }
}

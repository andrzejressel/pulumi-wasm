/// Manages a Custom Event Trigger inside an Azure Data Factory.
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
///   exampleTopic:
///     type: azure:eventgrid:Topic
///     name: example
///     properties:
///       name: example-topic
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleTriggerCustomEvent:
///     type: azure:datafactory:TriggerCustomEvent
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       eventgridTopicId: ${exampleTopic.id}
///       events:
///         - event1
///         - event2
///       subjectBeginsWith: abc
///       subjectEndsWith: xyz
///       annotations:
///         - example1
///         - example2
///         - example3
///       description: example description
///       pipelines:
///         - name: ${examplePipeline.name}
///           parameters:
///             Env: Prod
///       additionalProperties:
///         foo: foo1
///         bar: bar2
/// ```
///
/// ## Import
///
/// Data Factory Custom Event Trigger can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/triggerCustomEvent:TriggerCustomEvent example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/triggers/example
/// ```
///
pub mod trigger_custom_event {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerCustomEventArgs {
        /// Specifies if the Data Factory Custom Event Trigger is activated. Defaults to `true`.
        #[builder(into, default)]
        pub activated: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Custom Event Trigger.
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Custom Event Trigger.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Custom Event Trigger.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of Event Grid Topic in which event will be listened. Changing this forces a new resource.
        #[builder(into)]
        pub eventgrid_topic_id: pulumi_wasm_rust::Output<String>,
        /// List of events that will fire this trigger. At least one event must be specified.
        #[builder(into)]
        pub events: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the name of the Data Factory Custom Event Trigger. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `pipeline` blocks as defined below.
        #[builder(into)]
        pub pipelines: pulumi_wasm_rust::Output<
            Vec<super::super::types::datafactory::TriggerCustomEventPipeline>,
        >,
        /// The pattern that event subject starts with for trigger to fire.
        #[builder(into, default)]
        pub subject_begins_with: pulumi_wasm_rust::Output<Option<String>>,
        /// The pattern that event subject ends with for trigger to fire.
        #[builder(into, default)]
        pub subject_ends_with: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TriggerCustomEventResult {
        /// Specifies if the Data Factory Custom Event Trigger is activated. Defaults to `true`.
        pub activated: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Custom Event Trigger.
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Custom Event Trigger.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Custom Event Trigger.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of Event Grid Topic in which event will be listened. Changing this forces a new resource.
        pub eventgrid_topic_id: pulumi_wasm_rust::Output<String>,
        /// List of events that will fire this trigger. At least one event must be specified.
        pub events: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the name of the Data Factory Custom Event Trigger. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `pipeline` blocks as defined below.
        pub pipelines: pulumi_wasm_rust::Output<
            Vec<super::super::types::datafactory::TriggerCustomEventPipeline>,
        >,
        /// The pattern that event subject starts with for trigger to fire.
        pub subject_begins_with: pulumi_wasm_rust::Output<Option<String>>,
        /// The pattern that event subject ends with for trigger to fire.
        pub subject_ends_with: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TriggerCustomEventArgs) -> TriggerCustomEventResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let activated_binding = args.activated.get_inner();
        let additional_properties_binding = args.additional_properties.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let eventgrid_topic_id_binding = args.eventgrid_topic_id.get_inner();
        let events_binding = args.events.get_inner();
        let name_binding = args.name.get_inner();
        let pipelines_binding = args.pipelines.get_inner();
        let subject_begins_with_binding = args.subject_begins_with.get_inner();
        let subject_ends_with_binding = args.subject_ends_with.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/triggerCustomEvent:TriggerCustomEvent".into(),
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
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "eventgridTopicId".into(),
                    value: &eventgrid_topic_id_binding,
                },
                register_interface::ObjectField {
                    name: "events".into(),
                    value: &events_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pipelines".into(),
                    value: &pipelines_binding,
                },
                register_interface::ObjectField {
                    name: "subjectBeginsWith".into(),
                    value: &subject_begins_with_binding,
                },
                register_interface::ObjectField {
                    name: "subjectEndsWith".into(),
                    value: &subject_ends_with_binding,
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
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "eventgridTopicId".into(),
                },
                register_interface::ResultField {
                    name: "events".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pipelines".into(),
                },
                register_interface::ResultField {
                    name: "subjectBeginsWith".into(),
                },
                register_interface::ResultField {
                    name: "subjectEndsWith".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TriggerCustomEventResult {
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
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            eventgrid_topic_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventgridTopicId").unwrap(),
            ),
            events: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("events").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pipelines: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelines").unwrap(),
            ),
            subject_begins_with: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subjectBeginsWith").unwrap(),
            ),
            subject_ends_with: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subjectEndsWith").unwrap(),
            ),
        }
    }
}
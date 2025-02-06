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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerCustomEventArgs {
        /// Specifies if the Data Factory Custom Event Trigger is activated. Defaults to `true`.
        #[builder(into, default)]
        pub activated: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Custom Event Trigger.
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Custom Event Trigger.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Custom Event Trigger.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of Event Grid Topic in which event will be listened. Changing this forces a new resource.
        #[builder(into)]
        pub eventgrid_topic_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of events that will fire this trigger. At least one event must be specified.
        #[builder(into)]
        pub events: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Specifies the name of the Data Factory Custom Event Trigger. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `pipeline` blocks as defined below.
        #[builder(into)]
        pub pipelines: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::datafactory::TriggerCustomEventPipeline>,
        >,
        /// The pattern that event subject starts with for trigger to fire.
        #[builder(into, default)]
        pub subject_begins_with: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The pattern that event subject ends with for trigger to fire.
        #[builder(into, default)]
        pub subject_ends_with: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TriggerCustomEventResult {
        /// Specifies if the Data Factory Custom Event Trigger is activated. Defaults to `true`.
        pub activated: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A map of additional properties to associate with the Data Factory Custom Event Trigger.
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Custom Event Trigger.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of Data Factory in which to associate the Trigger with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Custom Event Trigger.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of Event Grid Topic in which event will be listened. Changing this forces a new resource.
        pub eventgrid_topic_id: pulumi_gestalt_rust::Output<String>,
        /// List of events that will fire this trigger. At least one event must be specified.
        pub events: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the name of the Data Factory Custom Event Trigger. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `pipeline` blocks as defined below.
        pub pipelines: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datafactory::TriggerCustomEventPipeline>,
        >,
        /// The pattern that event subject starts with for trigger to fire.
        pub subject_begins_with: pulumi_gestalt_rust::Output<Option<String>>,
        /// The pattern that event subject ends with for trigger to fire.
        pub subject_ends_with: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TriggerCustomEventArgs,
    ) -> TriggerCustomEventResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let activated_binding = args.activated.get_output(context).get_inner();
        let additional_properties_binding = args
            .additional_properties
            .get_output(context)
            .get_inner();
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let data_factory_id_binding = args
            .data_factory_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let eventgrid_topic_id_binding = args
            .eventgrid_topic_id
            .get_output(context)
            .get_inner();
        let events_binding = args.events.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let pipelines_binding = args.pipelines.get_output(context).get_inner();
        let subject_begins_with_binding = args
            .subject_begins_with
            .get_output(context)
            .get_inner();
        let subject_ends_with_binding = args
            .subject_ends_with
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/triggerCustomEvent:TriggerCustomEvent".into(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        TriggerCustomEventResult {
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
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            eventgrid_topic_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventgridTopicId"),
            ),
            events: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("events"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            pipelines: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pipelines"),
            ),
            subject_begins_with: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subjectBeginsWith"),
            ),
            subject_ends_with: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subjectEndsWith"),
            ),
        }
    }
}

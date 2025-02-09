/// Manages a Digital Twins Event Grid Endpoint.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example_resources")
///             .build_struct(),
///     );
///     let exampleEndpointEventGrid = endpoint_event_grid::create(
///         "exampleEndpointEventGrid",
///         EndpointEventGridArgs::builder()
///             .digital_twins_id("${exampleInstance.id}")
///             .eventgrid_topic_endpoint("${exampleTopic.endpoint}")
///             .eventgrid_topic_primary_access_key("${exampleTopic.primaryAccessKey}")
///             .eventgrid_topic_secondary_access_key("${exampleTopic.secondaryAccessKey}")
///             .name("example-EG")
///             .build_struct(),
///     );
///     let exampleInstance = instance::create(
///         "exampleInstance",
///         InstanceArgs::builder()
///             .location("${example.location}")
///             .name("example-DT")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleTopic = topic::create(
///         "exampleTopic",
///         TopicArgs::builder()
///             .location("${example.location}")
///             .name("example-topic")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Digital Twins Eventgrid Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:digitaltwins/endpointEventGrid:EndpointEventGrid example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DigitalTwins/digitalTwinsInstances/dt1/endpoints/ep1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_event_grid {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointEventGridArgs {
        /// The storage secret of the dead-lettering, whose format is `https://<storageAccountname>.blob.core.windows.net/<containerName>?<SASToken>`. When an endpoint can't deliver an event within a certain time period or after trying to deliver the event a certain number of times, it can send the undelivered event to a storage account.
        #[builder(into, default)]
        pub dead_letter_storage_secret: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The resource ID of the Digital Twins Instance. Changing this forces a new Digital Twins Eventgrid Endpoint to be created.
        #[builder(into)]
        pub digital_twins_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The endpoint of the Event Grid Topic.
        #[builder(into)]
        pub eventgrid_topic_endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The primary access key of the Event Grid Topic.
        #[builder(into)]
        pub eventgrid_topic_primary_access_key: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The secondary access key of the Event Grid Topic.
        #[builder(into)]
        pub eventgrid_topic_secondary_access_key: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The name which should be used for this Digital Twins Eventgrid Endpoint. Changing this forces a new Digital Twins Eventgrid Endpoint to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EndpointEventGridResult {
        /// The storage secret of the dead-lettering, whose format is `https://<storageAccountname>.blob.core.windows.net/<containerName>?<SASToken>`. When an endpoint can't deliver an event within a certain time period or after trying to deliver the event a certain number of times, it can send the undelivered event to a storage account.
        pub dead_letter_storage_secret: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource ID of the Digital Twins Instance. Changing this forces a new Digital Twins Eventgrid Endpoint to be created.
        pub digital_twins_id: pulumi_gestalt_rust::Output<String>,
        /// The endpoint of the Event Grid Topic.
        pub eventgrid_topic_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The primary access key of the Event Grid Topic.
        pub eventgrid_topic_primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary access key of the Event Grid Topic.
        pub eventgrid_topic_secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Digital Twins Eventgrid Endpoint. Changing this forces a new Digital Twins Eventgrid Endpoint to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointEventGridArgs,
    ) -> EndpointEventGridResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dead_letter_storage_secret_binding = args
            .dead_letter_storage_secret
            .get_output(context);
        let digital_twins_id_binding = args.digital_twins_id.get_output(context);
        let eventgrid_topic_endpoint_binding = args
            .eventgrid_topic_endpoint
            .get_output(context);
        let eventgrid_topic_primary_access_key_binding = args
            .eventgrid_topic_primary_access_key
            .get_output(context);
        let eventgrid_topic_secondary_access_key_binding = args
            .eventgrid_topic_secondary_access_key
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:digitaltwins/endpointEventGrid:EndpointEventGrid".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deadLetterStorageSecret".into(),
                    value: dead_letter_storage_secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "digitalTwinsId".into(),
                    value: digital_twins_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventgridTopicEndpoint".into(),
                    value: eventgrid_topic_endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventgridTopicPrimaryAccessKey".into(),
                    value: eventgrid_topic_primary_access_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventgridTopicSecondaryAccessKey".into(),
                    value: eventgrid_topic_secondary_access_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointEventGridResult {
            dead_letter_storage_secret: o.get_field("deadLetterStorageSecret"),
            digital_twins_id: o.get_field("digitalTwinsId"),
            eventgrid_topic_endpoint: o.get_field("eventgridTopicEndpoint"),
            eventgrid_topic_primary_access_key: o
                .get_field("eventgridTopicPrimaryAccessKey"),
            eventgrid_topic_secondary_access_key: o
                .get_field("eventgridTopicSecondaryAccessKey"),
            name: o.get_field("name"),
        }
    }
}

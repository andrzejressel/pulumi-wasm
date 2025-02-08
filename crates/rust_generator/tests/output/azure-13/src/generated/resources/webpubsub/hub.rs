/// Manages the hub settings for a Web Pubsub.
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
///             .location("east us")
///             .name("terraform-webpubsub")
///             .build_struct(),
///     );
///     let exampleHub = hub::create(
///         "exampleHub",
///         HubArgs::builder()
///             .anonymous_connections_enabled(true)
///             .event_handlers(
///                 vec![
///                     HubEventHandler::builder().systemEvents(vec!["connect",
///                     "connected",]).urlTemplate("https://test.com/api/{hub}/{event}")
///                     .userEventPattern("*").build_struct(), HubEventHandler::builder()
///                     .auth(HubEventHandlerAuth::builder()
///                     .managedIdentityId("${exampleUserAssignedIdentity.id}")
///                     .build_struct()).systemEvents(vec!["connected",])
///                     .urlTemplate("https://test.com/api/{hub}/{event}")
///                     .userEventPattern("event1, event2").build_struct(),
///                 ],
///             )
///             .event_listeners(
///                 vec![
///                     HubEventListener::builder().eventhubName("${test1.name}")
///                     .eventhubNamespaceName("${test.name}")
///                     .systemEventNameFilters(vec!["connected",])
///                     .userEventNameFilters(vec!["event1", "event2",]).build_struct(),
///                     HubEventListener::builder().eventhubName("${test1.name}")
///                     .eventhubNamespaceName("${test.name}")
///                     .systemEventNameFilters(vec!["connected",])
///                     .userEventNameFilters(vec!["*",]).build_struct(),
///                     HubEventListener::builder().eventhubName("${test1.name}")
///                     .eventhubNamespaceName("${test.name}")
///                     .systemEventNameFilters(vec!["connected",])
///                     .userEventNameFilters(vec!["event1",]).build_struct(),
///                 ],
///             )
///             .name("tfex_wpsh")
///             .web_pubsub_id("${exampleService.id}")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .capacity(1)
///             .location("${example.location}")
///             .name("tfex-webpubsub")
///             .resource_group_name("${example.name}")
///             .sku("Standard_S1")
///             .build_struct(),
///     );
///     let exampleUserAssignedIdentity = user_assigned_identity::create(
///         "exampleUserAssignedIdentity",
///         UserAssignedIdentityArgs::builder()
///             .location("${example.location}")
///             .name("tfex-uai")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Web Pubsub Hub can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:webpubsub/hub:Hub example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SignalRService/webPubSub/webPubSub1/hubs/webPubSubhub1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hub {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HubArgs {
        /// Is anonymous connections are allowed for this hub? Defaults to `false`.
        /// Possible values are `true`, `false`.
        #[builder(into, default)]
        pub anonymous_connections_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An `event_handler` block as defined below.
        ///
        /// > **NOTE:** User can change the order of `event_handler` to change the priority accordingly.
        #[builder(into, default)]
        pub event_handlers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::webpubsub::HubEventHandler>>,
        >,
        /// An `event_listener` block as defined below.
        ///
        /// > **NOTE:**  The managed identity of Web PubSub service must be enabled and the identity must have the "Azure Event Hubs Data sender" role to access the Event Hub.
        #[builder(into, default)]
        pub event_listeners: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::webpubsub::HubEventListener>>,
        >,
        /// The name of the Web Pubsub hub service. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the id of the Web Pubsub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub web_pubsub_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HubResult {
        /// Is anonymous connections are allowed for this hub? Defaults to `false`.
        /// Possible values are `true`, `false`.
        pub anonymous_connections_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `event_handler` block as defined below.
        ///
        /// > **NOTE:** User can change the order of `event_handler` to change the priority accordingly.
        pub event_handlers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::webpubsub::HubEventHandler>>,
        >,
        /// An `event_listener` block as defined below.
        ///
        /// > **NOTE:**  The managed identity of Web PubSub service must be enabled and the identity must have the "Azure Event Hubs Data sender" role to access the Event Hub.
        pub event_listeners: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::webpubsub::HubEventListener>>,
        >,
        /// The name of the Web Pubsub hub service. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the id of the Web Pubsub. Changing this forces a new resource to be created.
        pub web_pubsub_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: HubArgs,
    ) -> HubResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let anonymous_connections_enabled_binding = args
            .anonymous_connections_enabled
            .get_output(context)
            .get_inner();
        let event_handlers_binding = args.event_handlers.get_output(context).get_inner();
        let event_listeners_binding = args
            .event_listeners
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let web_pubsub_id_binding = args.web_pubsub_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:webpubsub/hub:Hub".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "anonymousConnectionsEnabled".into(),
                    value: &anonymous_connections_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "eventHandlers".into(),
                    value: &event_handlers_binding,
                },
                register_interface::ObjectField {
                    name: "eventListeners".into(),
                    value: &event_listeners_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "webPubsubId".into(),
                    value: &web_pubsub_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HubResult {
            anonymous_connections_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("anonymousConnectionsEnabled"),
            ),
            event_handlers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventHandlers"),
            ),
            event_listeners: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventListeners"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            web_pubsub_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("webPubsubId"),
            ),
        }
    }
}

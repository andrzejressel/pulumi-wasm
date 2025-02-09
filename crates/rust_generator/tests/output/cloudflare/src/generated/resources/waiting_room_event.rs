/// Provides a Cloudflare Waiting Room Event resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = waiting_room_event::create(
///         "example",
///         WaitingRoomEventArgs::builder()
///             .event_end_time("2006-01-02T20:04:05Z")
///             .event_start_time("2006-01-02T15:04:05Z")
///             .name("foo")
///             .waiting_room_id("d41d8cd98f00b204e9800998ecf8427e")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Use the Zone ID, Waiting Room ID, and Event ID to import.
///
/// ```sh
/// $ pulumi import cloudflare:index/waitingRoomEvent:WaitingRoomEvent default <zone_id>/<waiting_room_id>/<waiting_room_event_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod waiting_room_event {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WaitingRoomEventArgs {
        /// This is a templated html file that will be rendered at the edge.
        #[builder(into, default)]
        pub custom_page_html: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A description to let users add more details about the event.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Disables automatic renewal of session cookies.
        #[builder(into, default)]
        pub disable_session_renewal: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// ISO 8601 timestamp that marks the end of the event. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub event_end_time: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ISO 8601 timestamp that marks the start of the event. Must occur at least 1 minute before `event_end_time`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub event_start_time: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique name to identify the event. Only alphanumeric characters, hyphens, and underscores are allowed. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of new users that will be let into the route every minute.
        #[builder(into, default)]
        pub new_users_per_minute: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// ISO 8601 timestamp that marks when to begin queueing all users before the event starts. Must occur at least 5 minutes before `event_start_time`.
        #[builder(into, default)]
        pub prequeue_start_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`.
        #[builder(into, default)]
        pub queueing_method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Lifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin.
        #[builder(into, default)]
        pub session_duration: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Users in the prequeue will be shuffled randomly at the `event_start_time`. Requires that `prequeue_start_time` is not null. Defaults to `false`.
        #[builder(into, default)]
        pub shuffle_at_event_start: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If suspended, the event is ignored and traffic will be handled based on the waiting room configuration.
        #[builder(into, default)]
        pub suspended: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The total number of active user sessions on the route at a point in time.
        #[builder(into, default)]
        pub total_active_users: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The Waiting Room ID the event should apply to. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub waiting_room_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WaitingRoomEventResult {
        /// Creation time.
        pub created_on: pulumi_gestalt_rust::Output<String>,
        /// This is a templated html file that will be rendered at the edge.
        pub custom_page_html: pulumi_gestalt_rust::Output<Option<String>>,
        /// A description to let users add more details about the event.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Disables automatic renewal of session cookies.
        pub disable_session_renewal: pulumi_gestalt_rust::Output<Option<bool>>,
        /// ISO 8601 timestamp that marks the end of the event. **Modifying this attribute will force creation of a new resource.**
        pub event_end_time: pulumi_gestalt_rust::Output<String>,
        /// ISO 8601 timestamp that marks the start of the event. Must occur at least 1 minute before `event_end_time`. **Modifying this attribute will force creation of a new resource.**
        pub event_start_time: pulumi_gestalt_rust::Output<String>,
        /// Last modified time.
        pub modified_on: pulumi_gestalt_rust::Output<String>,
        /// A unique name to identify the event. Only alphanumeric characters, hyphens, and underscores are allowed. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of new users that will be let into the route every minute.
        pub new_users_per_minute: pulumi_gestalt_rust::Output<Option<i32>>,
        /// ISO 8601 timestamp that marks when to begin queueing all users before the event starts. Must occur at least 5 minutes before `event_start_time`.
        pub prequeue_start_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// The queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`.
        pub queueing_method: pulumi_gestalt_rust::Output<Option<String>>,
        /// Lifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin.
        pub session_duration: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Users in the prequeue will be shuffled randomly at the `event_start_time`. Requires that `prequeue_start_time` is not null. Defaults to `false`.
        pub shuffle_at_event_start: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If suspended, the event is ignored and traffic will be handled based on the waiting room configuration.
        pub suspended: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The total number of active user sessions on the route at a point in time.
        pub total_active_users: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The Waiting Room ID the event should apply to. **Modifying this attribute will force creation of a new resource.**
        pub waiting_room_id: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WaitingRoomEventArgs,
    ) -> WaitingRoomEventResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_page_html_binding = args.custom_page_html.get_output(context);
        let description_binding = args.description.get_output(context);
        let disable_session_renewal_binding = args
            .disable_session_renewal
            .get_output(context);
        let event_end_time_binding = args.event_end_time.get_output(context);
        let event_start_time_binding = args.event_start_time.get_output(context);
        let name_binding = args.name.get_output(context);
        let new_users_per_minute_binding = args.new_users_per_minute.get_output(context);
        let prequeue_start_time_binding = args.prequeue_start_time.get_output(context);
        let queueing_method_binding = args.queueing_method.get_output(context);
        let session_duration_binding = args.session_duration.get_output(context);
        let shuffle_at_event_start_binding = args
            .shuffle_at_event_start
            .get_output(context);
        let suspended_binding = args.suspended.get_output(context);
        let total_active_users_binding = args.total_active_users.get_output(context);
        let waiting_room_id_binding = args.waiting_room_id.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoomEvent:WaitingRoomEvent".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customPageHtml".into(),
                    value: custom_page_html_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableSessionRenewal".into(),
                    value: disable_session_renewal_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventEndTime".into(),
                    value: event_end_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventStartTime".into(),
                    value: event_start_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "newUsersPerMinute".into(),
                    value: new_users_per_minute_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prequeueStartTime".into(),
                    value: prequeue_start_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queueingMethod".into(),
                    value: queueing_method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionDuration".into(),
                    value: session_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shuffleAtEventStart".into(),
                    value: shuffle_at_event_start_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "suspended".into(),
                    value: suspended_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "totalActiveUsers".into(),
                    value: total_active_users_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitingRoomId".into(),
                    value: waiting_room_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WaitingRoomEventResult {
            created_on: o.get_field("createdOn"),
            custom_page_html: o.get_field("customPageHtml"),
            description: o.get_field("description"),
            disable_session_renewal: o.get_field("disableSessionRenewal"),
            event_end_time: o.get_field("eventEndTime"),
            event_start_time: o.get_field("eventStartTime"),
            modified_on: o.get_field("modifiedOn"),
            name: o.get_field("name"),
            new_users_per_minute: o.get_field("newUsersPerMinute"),
            prequeue_start_time: o.get_field("prequeueStartTime"),
            queueing_method: o.get_field("queueingMethod"),
            session_duration: o.get_field("sessionDuration"),
            shuffle_at_event_start: o.get_field("shuffleAtEventStart"),
            suspended: o.get_field("suspended"),
            total_active_users: o.get_field("totalActiveUsers"),
            waiting_room_id: o.get_field("waitingRoomId"),
            zone_id: o.get_field("zoneId"),
        }
    }
}

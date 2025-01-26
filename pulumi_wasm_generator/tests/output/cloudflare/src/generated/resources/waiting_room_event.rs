/// Provides a Cloudflare Waiting Room Event resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod waiting_room_event {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WaitingRoomEventArgs {
        /// This is a templated html file that will be rendered at the edge.
        #[builder(into, default)]
        pub custom_page_html: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A description to let users add more details about the event.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Disables automatic renewal of session cookies.
        #[builder(into, default)]
        pub disable_session_renewal: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// ISO 8601 timestamp that marks the end of the event. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub event_end_time: pulumi_wasm_rust::InputOrOutput<String>,
        /// ISO 8601 timestamp that marks the start of the event. Must occur at least 1 minute before `event_end_time`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub event_start_time: pulumi_wasm_rust::InputOrOutput<String>,
        /// A unique name to identify the event. Only alphanumeric characters, hyphens, and underscores are allowed. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The number of new users that will be let into the route every minute.
        #[builder(into, default)]
        pub new_users_per_minute: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// ISO 8601 timestamp that marks when to begin queueing all users before the event starts. Must occur at least 5 minutes before `event_start_time`.
        #[builder(into, default)]
        pub prequeue_start_time: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`.
        #[builder(into, default)]
        pub queueing_method: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Lifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin.
        #[builder(into, default)]
        pub session_duration: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Users in the prequeue will be shuffled randomly at the `event_start_time`. Requires that `prequeue_start_time` is not null. Defaults to `false`.
        #[builder(into, default)]
        pub shuffle_at_event_start: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// If suspended, the event is ignored and traffic will be handled based on the waiting room configuration.
        #[builder(into, default)]
        pub suspended: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The total number of active user sessions on the route at a point in time.
        #[builder(into, default)]
        pub total_active_users: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The Waiting Room ID the event should apply to. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub waiting_room_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WaitingRoomEventResult {
        /// Creation time.
        pub created_on: pulumi_wasm_rust::Output<String>,
        /// This is a templated html file that will be rendered at the edge.
        pub custom_page_html: pulumi_wasm_rust::Output<Option<String>>,
        /// A description to let users add more details about the event.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Disables automatic renewal of session cookies.
        pub disable_session_renewal: pulumi_wasm_rust::Output<Option<bool>>,
        /// ISO 8601 timestamp that marks the end of the event. **Modifying this attribute will force creation of a new resource.**
        pub event_end_time: pulumi_wasm_rust::Output<String>,
        /// ISO 8601 timestamp that marks the start of the event. Must occur at least 1 minute before `event_end_time`. **Modifying this attribute will force creation of a new resource.**
        pub event_start_time: pulumi_wasm_rust::Output<String>,
        /// Last modified time.
        pub modified_on: pulumi_wasm_rust::Output<String>,
        /// A unique name to identify the event. Only alphanumeric characters, hyphens, and underscores are allowed. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of new users that will be let into the route every minute.
        pub new_users_per_minute: pulumi_wasm_rust::Output<Option<i32>>,
        /// ISO 8601 timestamp that marks when to begin queueing all users before the event starts. Must occur at least 5 minutes before `event_start_time`.
        pub prequeue_start_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`.
        pub queueing_method: pulumi_wasm_rust::Output<Option<String>>,
        /// Lifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin.
        pub session_duration: pulumi_wasm_rust::Output<Option<i32>>,
        /// Users in the prequeue will be shuffled randomly at the `event_start_time`. Requires that `prequeue_start_time` is not null. Defaults to `false`.
        pub shuffle_at_event_start: pulumi_wasm_rust::Output<Option<bool>>,
        /// If suspended, the event is ignored and traffic will be handled based on the waiting room configuration.
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
        /// The total number of active user sessions on the route at a point in time.
        pub total_active_users: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Waiting Room ID the event should apply to. **Modifying this attribute will force creation of a new resource.**
        pub waiting_room_id: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WaitingRoomEventArgs,
    ) -> WaitingRoomEventResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_page_html_binding = args
            .custom_page_html
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let disable_session_renewal_binding = args
            .disable_session_renewal
            .get_output(context)
            .get_inner();
        let event_end_time_binding = args.event_end_time.get_output(context).get_inner();
        let event_start_time_binding = args
            .event_start_time
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let new_users_per_minute_binding = args
            .new_users_per_minute
            .get_output(context)
            .get_inner();
        let prequeue_start_time_binding = args
            .prequeue_start_time
            .get_output(context)
            .get_inner();
        let queueing_method_binding = args
            .queueing_method
            .get_output(context)
            .get_inner();
        let session_duration_binding = args
            .session_duration
            .get_output(context)
            .get_inner();
        let shuffle_at_event_start_binding = args
            .shuffle_at_event_start
            .get_output(context)
            .get_inner();
        let suspended_binding = args.suspended.get_output(context).get_inner();
        let total_active_users_binding = args
            .total_active_users
            .get_output(context)
            .get_inner();
        let waiting_room_id_binding = args
            .waiting_room_id
            .get_output(context)
            .get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoomEvent:WaitingRoomEvent".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customPageHtml".into(),
                    value: &custom_page_html_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disableSessionRenewal".into(),
                    value: &disable_session_renewal_binding,
                },
                register_interface::ObjectField {
                    name: "eventEndTime".into(),
                    value: &event_end_time_binding,
                },
                register_interface::ObjectField {
                    name: "eventStartTime".into(),
                    value: &event_start_time_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "newUsersPerMinute".into(),
                    value: &new_users_per_minute_binding,
                },
                register_interface::ObjectField {
                    name: "prequeueStartTime".into(),
                    value: &prequeue_start_time_binding,
                },
                register_interface::ObjectField {
                    name: "queueingMethod".into(),
                    value: &queueing_method_binding,
                },
                register_interface::ObjectField {
                    name: "sessionDuration".into(),
                    value: &session_duration_binding,
                },
                register_interface::ObjectField {
                    name: "shuffleAtEventStart".into(),
                    value: &shuffle_at_event_start_binding,
                },
                register_interface::ObjectField {
                    name: "suspended".into(),
                    value: &suspended_binding,
                },
                register_interface::ObjectField {
                    name: "totalActiveUsers".into(),
                    value: &total_active_users_binding,
                },
                register_interface::ObjectField {
                    name: "waitingRoomId".into(),
                    value: &waiting_room_id_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdOn".into(),
                },
                register_interface::ResultField {
                    name: "customPageHtml".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disableSessionRenewal".into(),
                },
                register_interface::ResultField {
                    name: "eventEndTime".into(),
                },
                register_interface::ResultField {
                    name: "eventStartTime".into(),
                },
                register_interface::ResultField {
                    name: "modifiedOn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "newUsersPerMinute".into(),
                },
                register_interface::ResultField {
                    name: "prequeueStartTime".into(),
                },
                register_interface::ResultField {
                    name: "queueingMethod".into(),
                },
                register_interface::ResultField {
                    name: "sessionDuration".into(),
                },
                register_interface::ResultField {
                    name: "shuffleAtEventStart".into(),
                },
                register_interface::ResultField {
                    name: "suspended".into(),
                },
                register_interface::ResultField {
                    name: "totalActiveUsers".into(),
                },
                register_interface::ResultField {
                    name: "waitingRoomId".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WaitingRoomEventResult {
            created_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdOn").unwrap(),
            ),
            custom_page_html: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customPageHtml").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disable_session_renewal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableSessionRenewal").unwrap(),
            ),
            event_end_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventEndTime").unwrap(),
            ),
            event_start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventStartTime").unwrap(),
            ),
            modified_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modifiedOn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            new_users_per_minute: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("newUsersPerMinute").unwrap(),
            ),
            prequeue_start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prequeueStartTime").unwrap(),
            ),
            queueing_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queueingMethod").unwrap(),
            ),
            session_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionDuration").unwrap(),
            ),
            shuffle_at_event_start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shuffleAtEventStart").unwrap(),
            ),
            suspended: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suspended").unwrap(),
            ),
            total_active_users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalActiveUsers").unwrap(),
            ),
            waiting_room_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitingRoomId").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}

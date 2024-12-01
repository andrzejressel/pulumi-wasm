//! Provides a Cloudflare Waiting Room Event resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = waiting_room_event::create(
//!         "example",
//!         WaitingRoomEventArgs::builder()
//!             .eventEndTime("2006-01-02T20:04:05Z")
//!             .eventStartTime("2006-01-02T15:04:05Z")
//!             .name("foo")
//!             .waitingRoomId("d41d8cd98f00b204e9800998ecf8427e")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! Use the Zone ID, Waiting Room ID, and Event ID to import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/waitingRoomEvent:WaitingRoomEvent default <zone_id>/<waiting_room_id>/<waiting_room_event_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WaitingRoomEventArgs {
    /// This is a templated html file that will be rendered at the edge.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_page_html: pulumi_wasm_rust::Output<Option<String>>,
    /// A description to let users add more details about the event.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Disables automatic renewal of session cookies.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub disable_session_renewal: pulumi_wasm_rust::Output<Option<bool>>,
    /// ISO 8601 timestamp that marks the end of the event. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub event_end_time: pulumi_wasm_rust::Output<String>,
    /// ISO 8601 timestamp that marks the start of the event. Must occur at least 1 minute before `event_end_time`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub event_start_time: pulumi_wasm_rust::Output<String>,
    /// A unique name to identify the event. Only alphanumeric characters, hyphens, and underscores are allowed. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The number of new users that will be let into the route every minute.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub new_users_per_minute: pulumi_wasm_rust::Output<Option<i32>>,
    /// ISO 8601 timestamp that marks when to begin queueing all users before the event starts. Must occur at least 5 minutes before `event_start_time`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub prequeue_start_time: pulumi_wasm_rust::Output<Option<String>>,
    /// The queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub queueing_method: pulumi_wasm_rust::Output<Option<String>>,
    /// Lifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub session_duration: pulumi_wasm_rust::Output<Option<i32>>,
    /// Users in the prequeue will be shuffled randomly at the `event_start_time`. Requires that `prequeue_start_time` is not null. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub shuffle_at_event_start: pulumi_wasm_rust::Output<Option<bool>>,
    /// If suspended, the event is ignored and traffic will be handled based on the waiting room configuration.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    /// The total number of active user sessions on the route at a point in time.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub total_active_users: pulumi_wasm_rust::Output<Option<i32>>,
    /// The Waiting Room ID the event should apply to. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub waiting_room_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

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
pub fn create(name: &str, args: WaitingRoomEventArgs) -> WaitingRoomEventResult {

    let result = crate::bindings::pulumi::cloudflare::waiting_room_event::invoke(name, &crate::bindings::pulumi::cloudflare::waiting_room_event::Args {
        custom_page_html: &args.custom_page_html.get_inner(),
        description: &args.description.get_inner(),
        disable_session_renewal: &args.disable_session_renewal.get_inner(),
        event_end_time: &args.event_end_time.get_inner(),
        event_start_time: &args.event_start_time.get_inner(),
        name: &args.name.get_inner(),
        new_users_per_minute: &args.new_users_per_minute.get_inner(),
        prequeue_start_time: &args.prequeue_start_time.get_inner(),
        queueing_method: &args.queueing_method.get_inner(),
        session_duration: &args.session_duration.get_inner(),
        shuffle_at_event_start: &args.shuffle_at_event_start.get_inner(),
        suspended: &args.suspended.get_inner(),
        total_active_users: &args.total_active_users.get_inner(),
        waiting_room_id: &args.waiting_room_id.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    WaitingRoomEventResult {
        created_on: crate::into_domain(result.created_on),
        custom_page_html: crate::into_domain(result.custom_page_html),
        description: crate::into_domain(result.description),
        disable_session_renewal: crate::into_domain(result.disable_session_renewal),
        event_end_time: crate::into_domain(result.event_end_time),
        event_start_time: crate::into_domain(result.event_start_time),
        modified_on: crate::into_domain(result.modified_on),
        name: crate::into_domain(result.name),
        new_users_per_minute: crate::into_domain(result.new_users_per_minute),
        prequeue_start_time: crate::into_domain(result.prequeue_start_time),
        queueing_method: crate::into_domain(result.queueing_method),
        session_duration: crate::into_domain(result.session_duration),
        shuffle_at_event_start: crate::into_domain(result.shuffle_at_event_start),
        suspended: crate::into_domain(result.suspended),
        total_active_users: crate::into_domain(result.total_active_users),
        waiting_room_id: crate::into_domain(result.waiting_room_id),
        zone_id: crate::into_domain(result.zone_id),
    }
}

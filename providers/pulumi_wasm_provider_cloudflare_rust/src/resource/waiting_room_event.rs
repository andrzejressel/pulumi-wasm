//! Provides a Cloudflare Waiting Room Event resource.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! // Waiting Room Event
//! const example = new cloudflare.WaitingRoomEvent("example", {
//!     eventEndTime: "2006-01-02T20:04:05Z",
//!     eventStartTime: "2006-01-02T15:04:05Z",
//!     name: "foo",
//!     waitingRoomId: "d41d8cd98f00b204e9800998ecf8427e",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! # Waiting Room Event
//! example = cloudflare.WaitingRoomEvent("example",
//!     event_end_time="2006-01-02T20:04:05Z",
//!     event_start_time="2006-01-02T15:04:05Z",
//!     name="foo",
//!     waiting_room_id="d41d8cd98f00b204e9800998ecf8427e",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711")
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//!
//! return await Deployment.RunAsync(() =>
//! {
//!     // Waiting Room Event
//!     var example = new Cloudflare.WaitingRoomEvent("example", new()
//!     {
//!         EventEndTime = "2006-01-02T20:04:05Z",
//!         EventStartTime = "2006-01-02T15:04:05Z",
//!         Name = "foo",
//!         WaitingRoomId = "d41d8cd98f00b204e9800998ecf8427e",
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!     });
//!
//! });
//! ```
//! ### Go
//! ```go
//! package main
//!
//! import (
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//!
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		// Waiting Room Event
//! 		_, err := cloudflare.NewWaitingRoomEvent(ctx, "example", &cloudflare.WaitingRoomEventArgs{
//! 			EventEndTime:   pulumi.String("2006-01-02T20:04:05Z"),
//! 			EventStartTime: pulumi.String("2006-01-02T15:04:05Z"),
//! 			Name:           pulumi.String("foo"),
//! 			WaitingRoomId:  pulumi.String("d41d8cd98f00b204e9800998ecf8427e"),
//! 			ZoneId:         pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		return nil
//! 	})
//! }
//! ```
//! ### Java
//! ```java
//! package generated_program;
//!
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.cloudflare.WaitingRoomEvent;
//! import com.pulumi.cloudflare.WaitingRoomEventArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//!
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//!
//!     public static void stack(Context ctx) {
//!         // Waiting Room Event
//!         var example = new WaitingRoomEvent("example", WaitingRoomEventArgs.builder()        
//!             .eventEndTime("2006-01-02T20:04:05Z")
//!             .eventStartTime("2006-01-02T15:04:05Z")
//!             .name("foo")
//!             .waitingRoomId("d41d8cd98f00b204e9800998ecf8427e")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Waiting Room Event
//!   example:
//!     type: cloudflare:WaitingRoomEvent
//!     properties:
//!       eventEndTime: 2006-01-02T20:04:05Z
//!       eventStartTime: 2006-01-02T15:04:05Z
//!       name: foo
//!       waitingRoomId: d41d8cd98f00b204e9800998ecf8427e
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
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

pub struct WaitingRoomEventArgs {
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
    let result = crate::bindings::pulumi::cloudflare::waiting_room_event::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::waiting_room_event::Args {
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
        },
    );

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

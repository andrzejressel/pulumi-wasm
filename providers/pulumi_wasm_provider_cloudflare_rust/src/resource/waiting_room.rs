//! Provides a Cloudflare Waiting Room resource.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! // Waiting Room
//! const example = new cloudflare.WaitingRoom("example", {
//!     additionalRoutes: [
//!         {
//!             host: "shop1.example.com",
//!             path: "/example-path",
//!         },
//!         {
//!             host: "shop2.example.com",
//!         },
//!     ],
//!     cookieSuffix: "queue1",
//!     host: "foo.example.com",
//!     name: "foo",
//!     newUsersPerMinute: 200,
//!     path: "/",
//!     queueingStatusCode: 200,
//!     totalActiveUsers: 200,
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! # Waiting Room
//! example = cloudflare.WaitingRoom("example",
//!     additional_routes=[
//!         cloudflare.WaitingRoomAdditionalRouteArgs(
//!             host="shop1.example.com",
//!             path="/example-path",
//!         ),
//!         cloudflare.WaitingRoomAdditionalRouteArgs(
//!             host="shop2.example.com",
//!         ),
//!     ],
//!     cookie_suffix="queue1",
//!     host="foo.example.com",
//!     name="foo",
//!     new_users_per_minute=200,
//!     path="/",
//!     queueing_status_code=200,
//!     total_active_users=200,
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
//!     // Waiting Room
//!     var example = new Cloudflare.WaitingRoom("example", new()
//!     {
//!         AdditionalRoutes = new[]
//!         {
//!             new Cloudflare.Inputs.WaitingRoomAdditionalRouteArgs
//!             {
//!                 Host = "shop1.example.com",
//!                 Path = "/example-path",
//!             },
//!             new Cloudflare.Inputs.WaitingRoomAdditionalRouteArgs
//!             {
//!                 Host = "shop2.example.com",
//!             },
//!         },
//!         CookieSuffix = "queue1",
//!         Host = "foo.example.com",
//!         Name = "foo",
//!         NewUsersPerMinute = 200,
//!         Path = "/",
//!         QueueingStatusCode = 200,
//!         TotalActiveUsers = 200,
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
//! 		// Waiting Room
//! 		_, err := cloudflare.NewWaitingRoom(ctx, "example", &cloudflare.WaitingRoomArgs{
//! 			AdditionalRoutes: cloudflare.WaitingRoomAdditionalRouteArray{
//! 				&cloudflare.WaitingRoomAdditionalRouteArgs{
//! 					Host: pulumi.String("shop1.example.com"),
//! 					Path: pulumi.String("/example-path"),
//! 				},
//! 				&cloudflare.WaitingRoomAdditionalRouteArgs{
//! 					Host: pulumi.String("shop2.example.com"),
//! 				},
//! 			},
//! 			CookieSuffix:       pulumi.String("queue1"),
//! 			Host:               pulumi.String("foo.example.com"),
//! 			Name:               pulumi.String("foo"),
//! 			NewUsersPerMinute:  pulumi.Int(200),
//! 			Path:               pulumi.String("/"),
//! 			QueueingStatusCode: pulumi.Int(200),
//! 			TotalActiveUsers:   pulumi.Int(200),
//! 			ZoneId:             pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.WaitingRoom;
//! import com.pulumi.cloudflare.WaitingRoomArgs;
//! import com.pulumi.cloudflare.inputs.WaitingRoomAdditionalRouteArgs;
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
//!         // Waiting Room
//!         var example = new WaitingRoom("example", WaitingRoomArgs.builder()        
//!             .additionalRoutes(            
//!                 WaitingRoomAdditionalRouteArgs.builder()
//!                     .host("shop1.example.com")
//!                     .path("/example-path")
//!                     .build(),
//!                 WaitingRoomAdditionalRouteArgs.builder()
//!                     .host("shop2.example.com")
//!                     .build())
//!             .cookieSuffix("queue1")
//!             .host("foo.example.com")
//!             .name("foo")
//!             .newUsersPerMinute(200)
//!             .path("/")
//!             .queueingStatusCode(200)
//!             .totalActiveUsers(200)
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Waiting Room
//!   example:
//!     type: cloudflare:WaitingRoom
//!     properties:
//!       additionalRoutes:
//!         - host: shop1.example.com
//!           path: /example-path
//!         - host: shop2.example.com
//!       cookieSuffix: queue1
//!       host: foo.example.com
//!       name: foo
//!       newUsersPerMinute: 200
//!       path: /
//!       queueingStatusCode: 200
//!       totalActiveUsers: 200
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! Use the Zone ID and Waiting Room ID to import.
//!
//! ```sh
//! $ pulumi import cloudflare:index/waitingRoom:WaitingRoom default <zone_id>/<waiting_room_id>
//! ```
//!

pub struct WaitingRoomArgs {
    /// A list of additional hostname and paths combination to be applied on the waiting room.
    pub additional_routes:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomAdditionalRoute>>>,
    /// A cookie suffix to be appended to the Cloudflare waiting room cookie name.
    pub cookie_suffix: pulumi_wasm_rust::Output<Option<String>>,
    /// This is a templated html file that will be rendered at the edge.
    pub custom_page_html: pulumi_wasm_rust::Output<Option<String>>,
    /// The language to use for the default waiting room page. Available values: `de-DE`, `es-ES`, `en-US`, `fr-FR`, `id-ID`, `it-IT`, `ja-JP`, `ko-KR`, `nl-NL`, `pl-PL`, `pt-BR`, `tr-TR`, `zh-CN`, `zh-TW`, `ru-RU`, `fa-IR`. Defaults to `en-US`.
    pub default_template_language: pulumi_wasm_rust::Output<Option<String>>,
    /// A description to add more details about the waiting room.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Disables automatic renewal of session cookies.
    pub disable_session_renewal: pulumi_wasm_rust::Output<Option<bool>>,
    /// The additional host name for which the waiting room to be applied on (no wildcards).
    pub host: pulumi_wasm_rust::Output<String>,
    /// If true, requests to the waiting room with the header `Accept: application/json` will receive a JSON response object.
    pub json_response_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// A unique name to identify the waiting room. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// The number of new users that will be let into the route every minute.
    pub new_users_per_minute: pulumi_wasm_rust::Output<i32>,
    /// The path within the additional host to enable the waiting room on. Defaults to `/`.
    pub path: pulumi_wasm_rust::Output<Option<String>>,
    /// If queue_all is true, then all traffic will be sent to the waiting room.
    pub queue_all: pulumi_wasm_rust::Output<Option<bool>>,
    /// The queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`. Defaults to `fifo`.
    pub queueing_method: pulumi_wasm_rust::Output<Option<String>>,
    /// HTTP status code returned to a user while in the queue. Defaults to `200`.
    pub queueing_status_code: pulumi_wasm_rust::Output<Option<i32>>,
    /// Lifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin. Defaults to `5`.
    pub session_duration: pulumi_wasm_rust::Output<Option<i32>>,
    /// Suspends the waiting room.
    pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    /// The total number of active user sessions on the route at a point in time.
    pub total_active_users: pulumi_wasm_rust::Output<i32>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WaitingRoomResult {
    /// A list of additional hostname and paths combination to be applied on the waiting room.
    pub additional_routes:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomAdditionalRoute>>>,
    /// A cookie suffix to be appended to the Cloudflare waiting room cookie name.
    pub cookie_suffix: pulumi_wasm_rust::Output<Option<String>>,
    /// This is a templated html file that will be rendered at the edge.
    pub custom_page_html: pulumi_wasm_rust::Output<Option<String>>,
    /// The language to use for the default waiting room page. Available values: `de-DE`, `es-ES`, `en-US`, `fr-FR`, `id-ID`, `it-IT`, `ja-JP`, `ko-KR`, `nl-NL`, `pl-PL`, `pt-BR`, `tr-TR`, `zh-CN`, `zh-TW`, `ru-RU`, `fa-IR`. Defaults to `en-US`.
    pub default_template_language: pulumi_wasm_rust::Output<Option<String>>,
    /// A description to add more details about the waiting room.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Disables automatic renewal of session cookies.
    pub disable_session_renewal: pulumi_wasm_rust::Output<Option<bool>>,
    /// The additional host name for which the waiting room to be applied on (no wildcards).
    pub host: pulumi_wasm_rust::Output<String>,
    /// If true, requests to the waiting room with the header `Accept: application/json` will receive a JSON response object.
    pub json_response_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// A unique name to identify the waiting room. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// The number of new users that will be let into the route every minute.
    pub new_users_per_minute: pulumi_wasm_rust::Output<i32>,
    /// The path within the additional host to enable the waiting room on. Defaults to `/`.
    pub path: pulumi_wasm_rust::Output<Option<String>>,
    /// If queue_all is true, then all traffic will be sent to the waiting room.
    pub queue_all: pulumi_wasm_rust::Output<Option<bool>>,
    /// The queueing method used by the waiting room. Available values: `fifo`, `random`, `passthrough`, `reject`. Defaults to `fifo`.
    pub queueing_method: pulumi_wasm_rust::Output<Option<String>>,
    /// HTTP status code returned to a user while in the queue. Defaults to `200`.
    pub queueing_status_code: pulumi_wasm_rust::Output<Option<i32>>,
    /// Lifetime of a cookie (in minutes) set by Cloudflare for users who get access to the origin. Defaults to `5`.
    pub session_duration: pulumi_wasm_rust::Output<Option<i32>>,
    /// Suspends the waiting room.
    pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    /// The total number of active user sessions on the route at a point in time.
    pub total_active_users: pulumi_wasm_rust::Output<i32>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WaitingRoomArgs) -> WaitingRoomResult {
    let result = crate::bindings::pulumi::cloudflare::waiting_room::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::waiting_room::Args {
            additional_routes: args.additional_routes.get_inner(),
            cookie_suffix: args.cookie_suffix.get_inner(),
            custom_page_html: args.custom_page_html.get_inner(),
            default_template_language: args.default_template_language.get_inner(),
            description: args.description.get_inner(),
            disable_session_renewal: args.disable_session_renewal.get_inner(),
            host: args.host.get_inner(),
            json_response_enabled: args.json_response_enabled.get_inner(),
            name: args.name.get_inner(),
            new_users_per_minute: args.new_users_per_minute.get_inner(),
            path: args.path.get_inner(),
            queue_all: args.queue_all.get_inner(),
            queueing_method: args.queueing_method.get_inner(),
            queueing_status_code: args.queueing_status_code.get_inner(),
            session_duration: args.session_duration.get_inner(),
            suspended: args.suspended.get_inner(),
            total_active_users: args.total_active_users.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    WaitingRoomResult {
        additional_routes: crate::into_domain(result.additional_routes),
        cookie_suffix: crate::into_domain(result.cookie_suffix),
        custom_page_html: crate::into_domain(result.custom_page_html),
        default_template_language: crate::into_domain(result.default_template_language),
        description: crate::into_domain(result.description),
        disable_session_renewal: crate::into_domain(result.disable_session_renewal),
        host: crate::into_domain(result.host),
        json_response_enabled: crate::into_domain(result.json_response_enabled),
        name: crate::into_domain(result.name),
        new_users_per_minute: crate::into_domain(result.new_users_per_minute),
        path: crate::into_domain(result.path),
        queue_all: crate::into_domain(result.queue_all),
        queueing_method: crate::into_domain(result.queueing_method),
        queueing_status_code: crate::into_domain(result.queueing_status_code),
        session_duration: crate::into_domain(result.session_duration),
        suspended: crate::into_domain(result.suspended),
        total_active_users: crate::into_domain(result.total_active_users),
        zone_id: crate::into_domain(result.zone_id),
    }
}

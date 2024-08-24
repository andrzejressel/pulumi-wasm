//! Provides a Cloudflare Waiting Room Rules resource.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.WaitingRoomRules("example", {
//!     rules: [
//!         {
//!             action: "bypass_waiting_room",
//!             description: "bypass ip list",
//!             expression: "src.ip in {192.0.2.0 192.0.2.1}",
//!             status: "enabled",
//!         },
//!         {
//!             action: "bypass_waiting_room",
//!             description: "bypass query string",
//!             expression: "http.request.uri.query contains \"bypass=true\"",
//!             status: "enabled",
//!         },
//!     ],
//!     waitingRoomId: "d41d8cd98f00b204e9800998ecf8427e",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.WaitingRoomRules("example",
//!     rules=[
//!         cloudflare.WaitingRoomRulesRuleArgs(
//!             action="bypass_waiting_room",
//!             description="bypass ip list",
//!             expression="src.ip in {192.0.2.0 192.0.2.1}",
//!             status="enabled",
//!         ),
//!         cloudflare.WaitingRoomRulesRuleArgs(
//!             action="bypass_waiting_room",
//!             description="bypass query string",
//!             expression="http.request.uri.query contains \"bypass=true\"",
//!             status="enabled",
//!         ),
//!     ],
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
//!     var example = new Cloudflare.WaitingRoomRules("example", new()
//!     {
//!         Rules = new[]
//!         {
//!             new Cloudflare.Inputs.WaitingRoomRulesRuleArgs
//!             {
//!                 Action = "bypass_waiting_room",
//!                 Description = "bypass ip list",
//!                 Expression = "src.ip in {192.0.2.0 192.0.2.1}",
//!                 Status = "enabled",
//!             },
//!             new Cloudflare.Inputs.WaitingRoomRulesRuleArgs
//!             {
//!                 Action = "bypass_waiting_room",
//!                 Description = "bypass query string",
//!                 Expression = "http.request.uri.query contains \"bypass=true\"",
//!                 Status = "enabled",
//!             },
//!         },
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
//! 		_, err := cloudflare.NewWaitingRoomRules(ctx, "example", &cloudflare.WaitingRoomRulesArgs{
//! 			Rules: cloudflare.WaitingRoomRulesRuleArray{
//! 				&cloudflare.WaitingRoomRulesRuleArgs{
//! 					Action:      pulumi.String("bypass_waiting_room"),
//! 					Description: pulumi.String("bypass ip list"),
//! 					Expression:  pulumi.String("src.ip in {192.0.2.0 192.0.2.1}"),
//! 					Status:      pulumi.String("enabled"),
//! 				},
//! 				&cloudflare.WaitingRoomRulesRuleArgs{
//! 					Action:      pulumi.String("bypass_waiting_room"),
//! 					Description: pulumi.String("bypass query string"),
//! 					Expression:  pulumi.String("http.request.uri.query contains \"bypass=true\""),
//! 					Status:      pulumi.String("enabled"),
//! 				},
//! 			},
//! 			WaitingRoomId: pulumi.String("d41d8cd98f00b204e9800998ecf8427e"),
//! 			ZoneId:        pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.WaitingRoomRules;
//! import com.pulumi.cloudflare.WaitingRoomRulesArgs;
//! import com.pulumi.cloudflare.inputs.WaitingRoomRulesRuleArgs;
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
//!         var example = new WaitingRoomRules("example", WaitingRoomRulesArgs.builder()        
//!             .rules(            
//!                 WaitingRoomRulesRuleArgs.builder()
//!                     .action("bypass_waiting_room")
//!                     .description("bypass ip list")
//!                     .expression("src.ip in {192.0.2.0 192.0.2.1}")
//!                     .status("enabled")
//!                     .build(),
//!                 WaitingRoomRulesRuleArgs.builder()
//!                     .action("bypass_waiting_room")
//!                     .description("bypass query string")
//!                     .expression("http.request.uri.query contains \"bypass=true\"")
//!                     .status("enabled")
//!                     .build())
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
//!   example:
//!     type: cloudflare:WaitingRoomRules
//!     properties:
//!       rules:
//!         - action: bypass_waiting_room
//!           description: bypass ip list
//!           expression: src.ip in {192.0.2.0 192.0.2.1}
//!           status: enabled
//!         - action: bypass_waiting_room
//!           description: bypass query string
//!           expression: http.request.uri.query contains "bypass=true"
//!           status: enabled
//!       waitingRoomId: d41d8cd98f00b204e9800998ecf8427e
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/waitingRoomRules:WaitingRoomRules default <zone_id>/<waiting_room_id>
//! ```
//!

pub struct WaitingRoomRulesArgs {
    /// List of rules to apply to the ruleset.
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomRulesRule>>>,
    /// The Waiting Room ID the rules should apply to. **Modifying this attribute will force creation of a new resource.**
    pub waiting_room_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WaitingRoomRulesResult {
    /// List of rules to apply to the ruleset.
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomRulesRule>>>,
    /// The Waiting Room ID the rules should apply to. **Modifying this attribute will force creation of a new resource.**
    pub waiting_room_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WaitingRoomRulesArgs) -> WaitingRoomRulesResult {
    let result = crate::bindings::pulumi::cloudflare::waiting_room_rules::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::waiting_room_rules::Args {
            rules: args.rules.get_inner(),
            waiting_room_id: args.waiting_room_id.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    WaitingRoomRulesResult {
        rules: crate::into_domain(result.rules),
        waiting_room_id: crate::into_domain(result.waiting_room_id),
        zone_id: crate::into_domain(result.zone_id),
    }
}

//! Provides a resource for managing Email Routing Addresses catch all behaviour.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.EmailRoutingCatchAll("example", {
//!     actions: [{
//!         type: "forward",
//!         values: ["destinationaddress@example.net"],
//!     }],
//!     enabled: true,
//!     matchers: [{
//!         type: "all",
//!     }],
//!     name: "example catch all",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.EmailRoutingCatchAll("example",
//!     actions=[cloudflare.EmailRoutingCatchAllActionArgs(
//!         type="forward",
//!         values=["destinationaddress@example.net"],
//!     )],
//!     enabled=True,
//!     matchers=[cloudflare.EmailRoutingCatchAllMatcherArgs(
//!         type="all",
//!     )],
//!     name="example catch all",
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
//!     var example = new Cloudflare.EmailRoutingCatchAll("example", new()
//!     {
//!         Actions = new[]
//!         {
//!             new Cloudflare.Inputs.EmailRoutingCatchAllActionArgs
//!             {
//!                 Type = "forward",
//!                 Values = new[]
//!                 {
//!                     "destinationaddress@example.net",
//!                 },
//!             },
//!         },
//!         Enabled = true,
//!         Matchers = new[]
//!         {
//!             new Cloudflare.Inputs.EmailRoutingCatchAllMatcherArgs
//!             {
//!                 Type = "all",
//!             },
//!         },
//!         Name = "example catch all",
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
//! 		_, err := cloudflare.NewEmailRoutingCatchAll(ctx, "example", &cloudflare.EmailRoutingCatchAllArgs{
//! 			Actions: cloudflare.EmailRoutingCatchAllActionArray{
//! 				&cloudflare.EmailRoutingCatchAllActionArgs{
//! 					Type: pulumi.String("forward"),
//! 					Values: pulumi.StringArray{
//! 						pulumi.String("destinationaddress@example.net"),
//! 					},
//! 				},
//! 			},
//! 			Enabled: pulumi.Bool(true),
//! 			Matchers: cloudflare.EmailRoutingCatchAllMatcherArray{
//! 				&cloudflare.EmailRoutingCatchAllMatcherArgs{
//! 					Type: pulumi.String("all"),
//! 				},
//! 			},
//! 			Name:   pulumi.String("example catch all"),
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.EmailRoutingCatchAll;
//! import com.pulumi.cloudflare.EmailRoutingCatchAllArgs;
//! import com.pulumi.cloudflare.inputs.EmailRoutingCatchAllActionArgs;
//! import com.pulumi.cloudflare.inputs.EmailRoutingCatchAllMatcherArgs;
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
//!         var example = new EmailRoutingCatchAll("example", EmailRoutingCatchAllArgs.builder()        
//!             .actions(EmailRoutingCatchAllActionArgs.builder()
//!                 .type("forward")
//!                 .values("destinationaddress@example.net")
//!                 .build())
//!             .enabled(true)
//!             .matchers(EmailRoutingCatchAllMatcherArgs.builder()
//!                 .type("all")
//!                 .build())
//!             .name("example catch all")
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
//!     type: cloudflare:EmailRoutingCatchAll
//!     properties:
//!       actions:
//!         - type: forward
//!           values:
//!             - destinationaddress@example.net
//!       enabled: true
//!       matchers:
//!         - type: all
//!       name: example catch all
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingCatchAllArgs {
    /// List actions patterns.
    #[builder(into)]
    pub actions: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllAction>>,
    /// Routing rule status.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Matching patterns to forward to your actions.
    #[builder(into)]
    pub matchers: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllMatcher>>,
    /// Routing rule name.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct EmailRoutingCatchAllResult {
    /// List actions patterns.
    pub actions: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllAction>>,
    /// Routing rule status.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Matching patterns to forward to your actions.
    pub matchers: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllMatcher>>,
    /// Routing rule name.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Routing rule identifier.
    pub tag: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: EmailRoutingCatchAllArgs) -> EmailRoutingCatchAllResult {
    let result = crate::bindings::pulumi::cloudflare::email_routing_catch_all::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::email_routing_catch_all::Args {
            actions: &args.actions.get_inner(),
            enabled: &args.enabled.get_inner(),
            matchers: &args.matchers.get_inner(),
            name: &args.name.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    EmailRoutingCatchAllResult {
        actions: crate::into_domain(result.actions),
        enabled: crate::into_domain(result.enabled),
        matchers: crate::into_domain(result.matchers),
        name: crate::into_domain(result.name),
        tag: crate::into_domain(result.tag),
        zone_id: crate::into_domain(result.zone_id),
    }
}

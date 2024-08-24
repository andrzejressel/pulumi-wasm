//! Provides a resource to manage User Agent Blocking Rules.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example1 = new cloudflare.UserAgentBlockingRule("example1", {
//!     configuration: {
//!         target: "ua",
//!         value: "Chrome",
//!     },
//!     description: "My description 1",
//!     mode: "js_challenge",
//!     paused: false,
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! const example2 = new cloudflare.UserAgentBlockingRule("example2", {
//!     configuration: {
//!         target: "ua",
//!         value: "Mozilla",
//!     },
//!     description: "My description 22",
//!     mode: "challenge",
//!     paused: true,
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example1 = cloudflare.UserAgentBlockingRule("example1",
//!     configuration=cloudflare.UserAgentBlockingRuleConfigurationArgs(
//!         target="ua",
//!         value="Chrome",
//!     ),
//!     description="My description 1",
//!     mode="js_challenge",
//!     paused=False,
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711")
//! example2 = cloudflare.UserAgentBlockingRule("example2",
//!     configuration=cloudflare.UserAgentBlockingRuleConfigurationArgs(
//!         target="ua",
//!         value="Mozilla",
//!     ),
//!     description="My description 22",
//!     mode="challenge",
//!     paused=True,
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
//!     var example1 = new Cloudflare.UserAgentBlockingRule("example1", new()
//!     {
//!         Configuration = new Cloudflare.Inputs.UserAgentBlockingRuleConfigurationArgs
//!         {
//!             Target = "ua",
//!             Value = "Chrome",
//!         },
//!         Description = "My description 1",
//!         Mode = "js_challenge",
//!         Paused = false,
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!     });
//!
//!     var example2 = new Cloudflare.UserAgentBlockingRule("example2", new()
//!     {
//!         Configuration = new Cloudflare.Inputs.UserAgentBlockingRuleConfigurationArgs
//!         {
//!             Target = "ua",
//!             Value = "Mozilla",
//!         },
//!         Description = "My description 22",
//!         Mode = "challenge",
//!         Paused = true,
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
//! 		_, err := cloudflare.NewUserAgentBlockingRule(ctx, "example1", &cloudflare.UserAgentBlockingRuleArgs{
//! 			Configuration: &cloudflare.UserAgentBlockingRuleConfigurationArgs{
//! 				Target: pulumi.String("ua"),
//! 				Value:  pulumi.String("Chrome"),
//! 			},
//! 			Description: pulumi.String("My description 1"),
//! 			Mode:        pulumi.String("js_challenge"),
//! 			Paused:      pulumi.Bool(false),
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewUserAgentBlockingRule(ctx, "example2", &cloudflare.UserAgentBlockingRuleArgs{
//! 			Configuration: &cloudflare.UserAgentBlockingRuleConfigurationArgs{
//! 				Target: pulumi.String("ua"),
//! 				Value:  pulumi.String("Mozilla"),
//! 			},
//! 			Description: pulumi.String("My description 22"),
//! 			Mode:        pulumi.String("challenge"),
//! 			Paused:      pulumi.Bool(true),
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.UserAgentBlockingRule;
//! import com.pulumi.cloudflare.UserAgentBlockingRuleArgs;
//! import com.pulumi.cloudflare.inputs.UserAgentBlockingRuleConfigurationArgs;
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
//!         var example1 = new UserAgentBlockingRule("example1", UserAgentBlockingRuleArgs.builder()        
//!             .configuration(UserAgentBlockingRuleConfigurationArgs.builder()
//!                 .target("ua")
//!                 .value("Chrome")
//!                 .build())
//!             .description("My description 1")
//!             .mode("js_challenge")
//!             .paused(false)
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!         var example2 = new UserAgentBlockingRule("example2", UserAgentBlockingRuleArgs.builder()        
//!             .configuration(UserAgentBlockingRuleConfigurationArgs.builder()
//!                 .target("ua")
//!                 .value("Mozilla")
//!                 .build())
//!             .description("My description 22")
//!             .mode("challenge")
//!             .paused(true)
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example1:
//!     type: cloudflare:UserAgentBlockingRule
//!     properties:
//!       configuration:
//!         target: ua
//!         value: Chrome
//!       description: My description 1
//!       mode: js_challenge
//!       paused: false
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   example2:
//!     type: cloudflare:UserAgentBlockingRule
//!     properties:
//!       configuration:
//!         target: ua
//!         value: Mozilla
//!       description: My description 22
//!       mode: challenge
//!       paused: true
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/userAgentBlockingRule:UserAgentBlockingRule example <zone_id>/<user_agent_blocking_rule_id>
//! ```
//!

pub struct UserAgentBlockingRuleArgs {
    /// The configuration object for the current rule.
    pub configuration: pulumi_wasm_rust::Output<crate::types::UserAgentBlockingRuleConfiguration>,
    /// An informative summary of the rule.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`.
    pub mode: pulumi_wasm_rust::Output<String>,
    /// When true, indicates that the rule is currently paused.
    pub paused: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct UserAgentBlockingRuleResult {
    /// The configuration object for the current rule.
    pub configuration: pulumi_wasm_rust::Output<crate::types::UserAgentBlockingRuleConfiguration>,
    /// An informative summary of the rule.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`.
    pub mode: pulumi_wasm_rust::Output<String>,
    /// When true, indicates that the rule is currently paused.
    pub paused: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: UserAgentBlockingRuleArgs) -> UserAgentBlockingRuleResult {
    let result = crate::bindings::pulumi::cloudflare::user_agent_blocking_rule::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::user_agent_blocking_rule::Args {
            configuration: args.configuration.get_inner(),
            description: args.description.get_inner(),
            mode: args.mode.get_inner(),
            paused: args.paused.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    UserAgentBlockingRuleResult {
        configuration: crate::into_domain(result.configuration),
        description: crate::into_domain(result.description),
        mode: crate::into_domain(result.mode),
        paused: crate::into_domain(result.paused),
        zone_id: crate::into_domain(result.zone_id),
    }
}

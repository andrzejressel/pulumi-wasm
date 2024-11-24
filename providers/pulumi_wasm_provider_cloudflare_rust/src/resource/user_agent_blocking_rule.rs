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
//! const example1 = new cloudflare.UserAgentBlockingRule("example_1", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     mode: "js_challenge",
//!     paused: false,
//!     description: "My description 1",
//!     configuration: {
//!         target: "ua",
//!         value: "Chrome",
//!     },
//! });
//! const example2 = new cloudflare.UserAgentBlockingRule("example_2", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     mode: "challenge",
//!     paused: true,
//!     description: "My description 22",
//!     configuration: {
//!         target: "ua",
//!         value: "Mozilla",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example1 = cloudflare.UserAgentBlockingRule("example_1",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     mode="js_challenge",
//!     paused=False,
//!     description="My description 1",
//!     configuration={
//!         "target": "ua",
//!         "value": "Chrome",
//!     })
//! example2 = cloudflare.UserAgentBlockingRule("example_2",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     mode="challenge",
//!     paused=True,
//!     description="My description 22",
//!     configuration={
//!         "target": "ua",
//!         "value": "Mozilla",
//!     })
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
//!     var example1 = new Cloudflare.UserAgentBlockingRule("example_1", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Mode = "js_challenge",
//!         Paused = false,
//!         Description = "My description 1",
//!         Configuration = new Cloudflare.Inputs.UserAgentBlockingRuleConfigurationArgs
//!         {
//!             Target = "ua",
//!             Value = "Chrome",
//!         },
//!     });
//! 
//!     var example2 = new Cloudflare.UserAgentBlockingRule("example_2", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Mode = "challenge",
//!         Paused = true,
//!         Description = "My description 22",
//!         Configuration = new Cloudflare.Inputs.UserAgentBlockingRuleConfigurationArgs
//!         {
//!             Target = "ua",
//!             Value = "Mozilla",
//!         },
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
//! 		_, err := cloudflare.NewUserAgentBlockingRule(ctx, "example_1", &cloudflare.UserAgentBlockingRuleArgs{
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Mode:        pulumi.String("js_challenge"),
//! 			Paused:      pulumi.Bool(false),
//! 			Description: pulumi.String("My description 1"),
//! 			Configuration: &cloudflare.UserAgentBlockingRuleConfigurationArgs{
//! 				Target: pulumi.String("ua"),
//! 				Value:  pulumi.String("Chrome"),
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewUserAgentBlockingRule(ctx, "example_2", &cloudflare.UserAgentBlockingRuleArgs{
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Mode:        pulumi.String("challenge"),
//! 			Paused:      pulumi.Bool(true),
//! 			Description: pulumi.String("My description 22"),
//! 			Configuration: &cloudflare.UserAgentBlockingRuleConfigurationArgs{
//! 				Target: pulumi.String("ua"),
//! 				Value:  pulumi.String("Mozilla"),
//! 			},
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
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .mode("js_challenge")
//!             .paused(false)
//!             .description("My description 1")
//!             .configuration(UserAgentBlockingRuleConfigurationArgs.builder()
//!                 .target("ua")
//!                 .value("Chrome")
//!                 .build())
//!             .build());
//! 
//!         var example2 = new UserAgentBlockingRule("example2", UserAgentBlockingRuleArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .mode("challenge")
//!             .paused(true)
//!             .description("My description 22")
//!             .configuration(UserAgentBlockingRuleConfigurationArgs.builder()
//!                 .target("ua")
//!                 .value("Mozilla")
//!                 .build())
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
//!     name: example_1
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       mode: js_challenge
//!       paused: false
//!       description: My description 1
//!       configuration:
//!         target: ua
//!         value: Chrome
//!   example2:
//!     type: cloudflare:UserAgentBlockingRule
//!     name: example_2
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       mode: challenge
//!       paused: true
//!       description: My description 22
//!       configuration:
//!         target: ua
//!         value: Mozilla
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/userAgentBlockingRule:UserAgentBlockingRule example <zone_id>/<user_agent_blocking_rule_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct UserAgentBlockingRuleArgs {
    /// The configuration object for the current rule.
    #[builder(into)]
    pub configuration: pulumi_wasm_rust::Output<crate::types::UserAgentBlockingRuleConfiguration>,
    /// An informative summary of the rule.
    #[builder(into)]
    pub description: pulumi_wasm_rust::Output<String>,
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`.
    #[builder(into)]
    pub mode: pulumi_wasm_rust::Output<String>,
    /// When true, indicates that the rule is currently paused.
    #[builder(into)]
    pub paused: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
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

    let result = crate::bindings::pulumi::cloudflare::user_agent_blocking_rule::invoke(name, &crate::bindings::pulumi::cloudflare::user_agent_blocking_rule::Args {
        configuration: &args.configuration.get_inner(),
        description: &args.description.get_inner(),
        mode: &args.mode.get_inner(),
        paused: &args.paused.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    UserAgentBlockingRuleResult {
        configuration: crate::into_domain(result.configuration),
        description: crate::into_domain(result.description),
        mode: crate::into_domain(result.mode),
        paused: crate::into_domain(result.paused),
        zone_id: crate::into_domain(result.zone_id),
    }
}

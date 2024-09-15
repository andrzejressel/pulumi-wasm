//! Provides a Cloudflare page rule resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Add a page rule to the domain
//! const foobar = new cloudflare.PageRule("foobar", {
//!     zoneId: _var.cloudflare_zone_id,
//!     target: `sub.${_var.cloudflare_zone}/page`,
//!     priority: 1,
//!     actions: {
//!         ssl: "flexible",
//!         emailObfuscation: "on",
//!         minifies: [{
//!             html: "off",
//!             css: "on",
//!             js: "on",
//!         }],
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Add a page rule to the domain
//! foobar = cloudflare.PageRule("foobar",
//!     zone_id=var["cloudflare_zone_id"],
//!     target=f"sub.{var['cloudflare_zone']}/page",
//!     priority=1,
//!     actions=cloudflare.PageRuleActionsArgs(
//!         ssl="flexible",
//!         email_obfuscation="on",
//!         minifies=[cloudflare.PageRuleActionsMinifyArgs(
//!             html="off",
//!             css="on",
//!             js="on",
//!         )],
//!     ))
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
//!     // Add a page rule to the domain
//!     var foobar = new Cloudflare.PageRule("foobar", new()
//!     {
//!         ZoneId = @var.Cloudflare_zone_id,
//!         Target = $"sub.{@var.Cloudflare_zone}/page",
//!         Priority = 1,
//!         Actions = new Cloudflare.Inputs.PageRuleActionsArgs
//!         {
//!             Ssl = "flexible",
//!             EmailObfuscation = "on",
//!             Minifies = new[]
//!             {
//!                 new Cloudflare.Inputs.PageRuleActionsMinifyArgs
//!                 {
//!                     Html = "off",
//!                     Css = "on",
//!                     Js = "on",
//!                 },
//!             },
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
//! 	"fmt"
//! 
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		// Add a page rule to the domain
//! 		_, err := cloudflare.NewPageRule(ctx, "foobar", &cloudflare.PageRuleArgs{
//! 			ZoneId:   pulumi.Any(_var.Cloudflare_zone_id),
//! 			Target:   pulumi.String(fmt.Sprintf("sub.%v/page", _var.Cloudflare_zone)),
//! 			Priority: pulumi.Int(1),
//! 			Actions: &cloudflare.PageRuleActionsArgs{
//! 				Ssl:              pulumi.String("flexible"),
//! 				EmailObfuscation: pulumi.String("on"),
//! 				Minifies: cloudflare.PageRuleActionsMinifyArray{
//! 					&cloudflare.PageRuleActionsMinifyArgs{
//! 						Html: pulumi.String("off"),
//! 						Css:  pulumi.String("on"),
//! 						Js:   pulumi.String("on"),
//! 					},
//! 				},
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
//! import com.pulumi.cloudflare.PageRule;
//! import com.pulumi.cloudflare.PageRuleArgs;
//! import com.pulumi.cloudflare.inputs.PageRuleActionsArgs;
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
//!         // Add a page rule to the domain
//!         var foobar = new PageRule("foobar", PageRuleArgs.builder()        
//!             .zoneId(var_.cloudflare_zone_id())
//!             .target(String.format("sub.%s/page", var_.cloudflare_zone()))
//!             .priority(1)
//!             .actions(PageRuleActionsArgs.builder()
//!                 .ssl("flexible")
//!                 .emailObfuscation("on")
//!                 .minifies(PageRuleActionsMinifyArgs.builder()
//!                     .html("off")
//!                     .css("on")
//!                     .js("on")
//!                     .build())
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Add a page rule to the domain
//!   foobar:
//!     type: cloudflare:PageRule
//!     properties:
//!       zoneId: ${var.cloudflare_zone_id}
//!       target: sub.${var.cloudflare_zone}/page
//!       priority: 1
//!       actions:
//!         ssl: flexible
//!         emailObfuscation: on
//!         minifies:
//!           - html: off
//!             css: on
//!             js: on
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! Page rules can be imported using a composite ID formed of zone ID and page rule ID, e.g.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/pageRule:PageRule default d41d8cd98f00b204e9800998ecf8427e/ch8374ftwdghsif43
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct PageRuleArgs {
    /// The actions taken by the page rule, options given below.
    #[builder(into)]
    pub actions: pulumi_wasm_rust::Output<crate::types::PageRuleActions>,
    /// The priority of the page rule among others for this target, the higher the number the higher the priority as per [API documentation](https://api.cloudflare.com/#page-rules-for-a-zone-create-page-rule).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Whether the page rule is active or disabled.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub status: pulumi_wasm_rust::Output<Option<String>>,
    /// The URL pattern to target with the page rule.
    #[builder(into)]
    pub target: pulumi_wasm_rust::Output<String>,
    /// The DNS zone ID to which the page rule should be added.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct PageRuleResult {
    /// The actions taken by the page rule, options given below.
    pub actions: pulumi_wasm_rust::Output<crate::types::PageRuleActions>,
    /// The priority of the page rule among others for this target, the higher the number the higher the priority as per [API documentation](https://api.cloudflare.com/#page-rules-for-a-zone-create-page-rule).
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Whether the page rule is active or disabled.
    pub status: pulumi_wasm_rust::Output<Option<String>>,
    /// The URL pattern to target with the page rule.
    pub target: pulumi_wasm_rust::Output<String>,
    /// The DNS zone ID to which the page rule should be added.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: PageRuleArgs) -> PageRuleResult {

    let result = crate::bindings::pulumi::cloudflare::page_rule::invoke(name, &crate::bindings::pulumi::cloudflare::page_rule::Args {
        actions: &args.actions.get_inner(),
        priority: &args.priority.get_inner(),
        status: &args.status.get_inner(),
        target: &args.target.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    PageRuleResult {
        actions: crate::into_domain(result.actions),
        priority: crate::into_domain(result.priority),
        status: crate::into_domain(result.status),
        target: crate::into_domain(result.target),
        zone_id: crate::into_domain(result.zone_id),
    }
}

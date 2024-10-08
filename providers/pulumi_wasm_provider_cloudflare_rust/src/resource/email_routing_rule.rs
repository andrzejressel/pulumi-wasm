//! The [Email Routing Rule](https://developers.cloudflare.com/email-routing/setup/email-routing-addresses/#email-rule-actions) resource allows you to create and manage email routing rules for a zone.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const main = new cloudflare.EmailRoutingRule("main", {
//!     actions: [{
//!         type: "forward",
//!         values: ["destinationaddress@example.net"],
//!     }],
//!     enabled: true,
//!     matchers: [{
//!         field: "to",
//!         type: "literal",
//!         value: "test@example.com",
//!     }],
//!     name: "terraform rule",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! main = cloudflare.EmailRoutingRule("main",
//!     actions=[cloudflare.EmailRoutingRuleActionArgs(
//!         type="forward",
//!         values=["destinationaddress@example.net"],
//!     )],
//!     enabled=True,
//!     matchers=[cloudflare.EmailRoutingRuleMatcherArgs(
//!         field="to",
//!         type="literal",
//!         value="test@example.com",
//!     )],
//!     name="terraform rule",
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
//!     var main = new Cloudflare.EmailRoutingRule("main", new()
//!     {
//!         Actions = new[]
//!         {
//!             new Cloudflare.Inputs.EmailRoutingRuleActionArgs
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
//!             new Cloudflare.Inputs.EmailRoutingRuleMatcherArgs
//!             {
//!                 Field = "to",
//!                 Type = "literal",
//!                 Value = "test@example.com",
//!             },
//!         },
//!         Name = "terraform rule",
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
//! 		_, err := cloudflare.NewEmailRoutingRule(ctx, "main", &cloudflare.EmailRoutingRuleArgs{
//! 			Actions: cloudflare.EmailRoutingRuleActionArray{
//! 				&cloudflare.EmailRoutingRuleActionArgs{
//! 					Type: pulumi.String("forward"),
//! 					Values: pulumi.StringArray{
//! 						pulumi.String("destinationaddress@example.net"),
//! 					},
//! 				},
//! 			},
//! 			Enabled: pulumi.Bool(true),
//! 			Matchers: cloudflare.EmailRoutingRuleMatcherArray{
//! 				&cloudflare.EmailRoutingRuleMatcherArgs{
//! 					Field: pulumi.String("to"),
//! 					Type:  pulumi.String("literal"),
//! 					Value: pulumi.String("test@example.com"),
//! 				},
//! 			},
//! 			Name:   pulumi.String("terraform rule"),
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
//! import com.pulumi.cloudflare.EmailRoutingRule;
//! import com.pulumi.cloudflare.EmailRoutingRuleArgs;
//! import com.pulumi.cloudflare.inputs.EmailRoutingRuleActionArgs;
//! import com.pulumi.cloudflare.inputs.EmailRoutingRuleMatcherArgs;
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
//!         var main = new EmailRoutingRule("main", EmailRoutingRuleArgs.builder()        
//!             .actions(EmailRoutingRuleActionArgs.builder()
//!                 .type("forward")
//!                 .values("destinationaddress@example.net")
//!                 .build())
//!             .enabled(true)
//!             .matchers(EmailRoutingRuleMatcherArgs.builder()
//!                 .field("to")
//!                 .type("literal")
//!                 .value("test@example.com")
//!                 .build())
//!             .name("terraform rule")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   main:
//!     type: cloudflare:EmailRoutingRule
//!     properties:
//!       actions:
//!         - type: forward
//!           values:
//!             - destinationaddress@example.net
//!       enabled: true
//!       matchers:
//!         - field: to
//!           type: literal
//!           value: test@example.com
//!       name: terraform rule
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/emailRoutingRule:EmailRoutingRule example <zone_id>/<email_routing_rule_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingRuleArgs {
    /// Actions to take when a match is found.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub actions: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleAction>>>,
    /// Whether the email routing rule is enabled.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Matching patterns to forward to your actions.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub matchers: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleMatcher>>>,
    /// Routing rule name.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The priority of the email routing rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct EmailRoutingRuleResult {
    /// Actions to take when a match is found.
    pub actions: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleAction>>>,
    /// Whether the email routing rule is enabled.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Matching patterns to forward to your actions.
    pub matchers: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleMatcher>>>,
    /// Routing rule name.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The priority of the email routing rule.
    pub priority: pulumi_wasm_rust::Output<i32>,
    /// The tag of the email routing rule.
    pub tag: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: EmailRoutingRuleArgs) -> EmailRoutingRuleResult {

    let result = crate::bindings::pulumi::cloudflare::email_routing_rule::invoke(name, &crate::bindings::pulumi::cloudflare::email_routing_rule::Args {
        actions: &args.actions.get_inner(),
        enabled: &args.enabled.get_inner(),
        matchers: &args.matchers.get_inner(),
        name: &args.name.get_inner(),
        priority: &args.priority.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    EmailRoutingRuleResult {
        actions: crate::into_domain(result.actions),
        enabled: crate::into_domain(result.enabled),
        matchers: crate::into_domain(result.matchers),
        name: crate::into_domain(result.name),
        priority: crate::into_domain(result.priority),
        tag: crate::into_domain(result.tag),
        zone_id: crate::into_domain(result.zone_id),
    }
}

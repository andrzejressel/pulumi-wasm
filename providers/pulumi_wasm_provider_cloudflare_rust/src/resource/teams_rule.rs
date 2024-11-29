//! Provides a Cloudflare Teams rule resource. Teams rules comprise secure web gateway policies.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.TeamsRule("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "office",
//!     description: "desc",
//!     precedence: 1,
//!     action: "block",
//!     filters: ["http"],
//!     traffic: "http.request.uri == \"https://www.example.com/malicious\"",
//!     ruleSettings: {
//!         blockPageEnabled: true,
//!         blockPageReason: "access not permitted",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.TeamsRule("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="office",
//!     description="desc",
//!     precedence=1,
//!     action="block",
//!     filters=["http"],
//!     traffic="http.request.uri == \"https://www.example.com/malicious\"",
//!     rule_settings={
//!         "block_page_enabled": True,
//!         "block_page_reason": "access not permitted",
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
//!     var example = new Cloudflare.TeamsRule("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "office",
//!         Description = "desc",
//!         Precedence = 1,
//!         Action = "block",
//!         Filters = new[]
//!         {
//!             "http",
//!         },
//!         Traffic = "http.request.uri == \"https://www.example.com/malicious\"",
//!         RuleSettings = new Cloudflare.Inputs.TeamsRuleRuleSettingsArgs
//!         {
//!             BlockPageEnabled = true,
//!             BlockPageReason = "access not permitted",
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
//! 		_, err := cloudflare.NewTeamsRule(ctx, "example", &cloudflare.TeamsRuleArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:        pulumi.String("office"),
//! 			Description: pulumi.String("desc"),
//! 			Precedence:  pulumi.Int(1),
//! 			Action:      pulumi.String("block"),
//! 			Filters: pulumi.StringArray{
//! 				pulumi.String("http"),
//! 			},
//! 			Traffic: pulumi.String("http.request.uri == \"https://www.example.com/malicious\""),
//! 			RuleSettings: &cloudflare.TeamsRuleRuleSettingsArgs{
//! 				BlockPageEnabled: pulumi.Bool(true),
//! 				BlockPageReason:  pulumi.String("access not permitted"),
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
//! import com.pulumi.cloudflare.TeamsRule;
//! import com.pulumi.cloudflare.TeamsRuleArgs;
//! import com.pulumi.cloudflare.inputs.TeamsRuleRuleSettingsArgs;
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
//!         var example = new TeamsRule("example", TeamsRuleArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("office")
//!             .description("desc")
//!             .precedence(1)
//!             .action("block")
//!             .filters("http")
//!             .traffic("http.request.uri == \"https://www.example.com/malicious\"")
//!             .ruleSettings(TeamsRuleRuleSettingsArgs.builder()
//!                 .blockPageEnabled(true)
//!                 .blockPageReason("access not permitted")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:TeamsRule
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: office
//!       description: desc
//!       precedence: 1
//!       action: block
//!       filters:
//!         - http
//!       traffic: http.request.uri == "https://www.example.com/malicious"
//!       ruleSettings:
//!         blockPageEnabled: true
//!         blockPageReason: access not permitted
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/teamsRule:TeamsRule example <account_id>/<teams_rule_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct TeamsRuleArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`, `resolve`.
    #[builder(into)]
    pub action: pulumi_wasm_rust::Output<String>,
    /// The description of the teams rule.
    #[builder(into)]
    pub description: pulumi_wasm_rust::Output<String>,
    /// The wirefilter expression to be used for device_posture check matching.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub device_posture: pulumi_wasm_rust::Output<Option<String>>,
    /// Indicator of rule enablement.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The protocol or layer to evaluate the traffic and identity expressions.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The wirefilter expression to be used for identity matching.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub identity: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the teams rule.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The evaluation precedence of the teams rule.
    #[builder(into)]
    pub precedence: pulumi_wasm_rust::Output<i32>,
    /// Additional rule settings.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub rule_settings: pulumi_wasm_rust::Output<Option<crate::types::TeamsRuleRuleSettings>>,
    /// The wirefilter expression to be used for traffic matching.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub traffic: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct TeamsRuleResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`, `resolve`.
    pub action: pulumi_wasm_rust::Output<String>,
    /// The description of the teams rule.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The wirefilter expression to be used for device_posture check matching.
    pub device_posture: pulumi_wasm_rust::Output<String>,
    /// Indicator of rule enablement.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The protocol or layer to evaluate the traffic and identity expressions.
    pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The wirefilter expression to be used for identity matching.
    pub identity: pulumi_wasm_rust::Output<String>,
    /// The name of the teams rule.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The evaluation precedence of the teams rule.
    pub precedence: pulumi_wasm_rust::Output<i32>,
    /// Additional rule settings.
    pub rule_settings: pulumi_wasm_rust::Output<crate::types::TeamsRuleRuleSettings>,
    /// The wirefilter expression to be used for traffic matching.
    pub traffic: pulumi_wasm_rust::Output<String>,
    pub version: pulumi_wasm_rust::Output<i32>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TeamsRuleArgs) -> TeamsRuleResult {

    let result = crate::bindings::pulumi::cloudflare::teams_rule::invoke(name, &crate::bindings::pulumi::cloudflare::teams_rule::Args {
        account_id: &args.account_id.get_inner(),
        action: &args.action.get_inner(),
        description: &args.description.get_inner(),
        device_posture: &args.device_posture.get_inner(),
        enabled: &args.enabled.get_inner(),
        filters: &args.filters.get_inner(),
        identity: &args.identity.get_inner(),
        name: &args.name.get_inner(),
        precedence: &args.precedence.get_inner(),
        rule_settings: &args.rule_settings.get_inner(),
        traffic: &args.traffic.get_inner(),
    });

    TeamsRuleResult {
        account_id: crate::into_domain(result.account_id),
        action: crate::into_domain(result.action),
        description: crate::into_domain(result.description),
        device_posture: crate::into_domain(result.device_posture),
        enabled: crate::into_domain(result.enabled),
        filters: crate::into_domain(result.filters),
        identity: crate::into_domain(result.identity),
        name: crate::into_domain(result.name),
        precedence: crate::into_domain(result.precedence),
        rule_settings: crate::into_domain(result.rule_settings),
        traffic: crate::into_domain(result.traffic),
        version: crate::into_domain(result.version),
    }
}

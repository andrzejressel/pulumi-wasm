//! Provides a Cloudflare Web Analytics Rule resource.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const exampleWebAnalyticsSite = new cloudflare.WebAnalyticsSite("exampleWebAnalyticsSite", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     zoneTag: "0da42c8d2132a9ddaf714f9e7c920711",
//!     autoInstall: true,
//! });
//! const exampleWebAnalyticsRule = new cloudflare.WebAnalyticsRule("exampleWebAnalyticsRule", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     rulesetId: exampleWebAnalyticsSite.rulesetId,
//!     host: "*",
//!     paths: ["/excluded"],
//!     inclusive: false,
//!     isPaused: false,
//! }, {
//!     dependsOn: [exampleWebAnalyticsSite],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example_web_analytics_site = cloudflare.WebAnalyticsSite("exampleWebAnalyticsSite",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     zone_tag="0da42c8d2132a9ddaf714f9e7c920711",
//!     auto_install=True)
//! example_web_analytics_rule = cloudflare.WebAnalyticsRule("exampleWebAnalyticsRule",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     ruleset_id=example_web_analytics_site.ruleset_id,
//!     host="*",
//!     paths=["/excluded"],
//!     inclusive=False,
//!     is_paused=False,
//!     opts=pulumi.ResourceOptions(depends_on=[example_web_analytics_site]))
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
//!     var exampleWebAnalyticsSite = new Cloudflare.WebAnalyticsSite("exampleWebAnalyticsSite", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         ZoneTag = "0da42c8d2132a9ddaf714f9e7c920711",
//!         AutoInstall = true,
//!     });
//!
//!     var exampleWebAnalyticsRule = new Cloudflare.WebAnalyticsRule("exampleWebAnalyticsRule", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         RulesetId = exampleWebAnalyticsSite.RulesetId,
//!         Host = "*",
//!         Paths = new[]
//!         {
//!             "/excluded",
//!         },
//!         Inclusive = false,
//!         IsPaused = false,
//!     }, new CustomResourceOptions
//!     {
//!         DependsOn =
//!         {
//!             exampleWebAnalyticsSite,
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
//! 		exampleWebAnalyticsSite, err := cloudflare.NewWebAnalyticsSite(ctx, "exampleWebAnalyticsSite", &cloudflare.WebAnalyticsSiteArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			ZoneTag:     pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			AutoInstall: pulumi.Bool(true),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewWebAnalyticsRule(ctx, "exampleWebAnalyticsRule", &cloudflare.WebAnalyticsRuleArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			RulesetId: exampleWebAnalyticsSite.RulesetId,
//! 			Host:      pulumi.String("*"),
//! 			Paths: pulumi.StringArray{
//! 				pulumi.String("/excluded"),
//! 			},
//! 			Inclusive: pulumi.Bool(false),
//! 			IsPaused:  pulumi.Bool(false),
//! 		}, pulumi.DependsOn([]pulumi.Resource{
//! 			exampleWebAnalyticsSite,
//! 		}))
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
//! import com.pulumi.cloudflare.WebAnalyticsSite;
//! import com.pulumi.cloudflare.WebAnalyticsSiteArgs;
//! import com.pulumi.cloudflare.WebAnalyticsRule;
//! import com.pulumi.cloudflare.WebAnalyticsRuleArgs;
//! import com.pulumi.resources.CustomResourceOptions;
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
//!         var exampleWebAnalyticsSite = new WebAnalyticsSite("exampleWebAnalyticsSite", WebAnalyticsSiteArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .zoneTag("0da42c8d2132a9ddaf714f9e7c920711")
//!             .autoInstall(true)
//!             .build());
//!
//!         var exampleWebAnalyticsRule = new WebAnalyticsRule("exampleWebAnalyticsRule", WebAnalyticsRuleArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .rulesetId(exampleWebAnalyticsSite.rulesetId())
//!             .host("*")
//!             .paths("/excluded")
//!             .inclusive(false)
//!             .isPaused(false)
//!             .build(), CustomResourceOptions.builder()
//!                 .dependsOn(exampleWebAnalyticsSite)
//!                 .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   exampleWebAnalyticsSite:
//!     type: cloudflare:WebAnalyticsSite
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       zoneTag: 0da42c8d2132a9ddaf714f9e7c920711
//!       autoInstall: true
//!   exampleWebAnalyticsRule:
//!     type: cloudflare:WebAnalyticsRule
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       rulesetId: ${exampleWebAnalyticsSite.rulesetId}
//!       host: '*'
//!       paths:
//!         - /excluded
//!       inclusive: false
//!       isPaused: false
//!     options:
//!       dependson:
//!         - ${exampleWebAnalyticsSite}
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/webAnalyticsRule:WebAnalyticsRule example <account_id>/<ruleset_id>/<rule_id>
//! ```
//!

pub struct WebAnalyticsRuleArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The host to apply the rule to.
    pub host: pulumi_wasm_rust::Output<String>,
    /// Whether the rule includes or excludes the matched traffic from being measured in Web Analytics.
    pub inclusive: pulumi_wasm_rust::Output<bool>,
    /// Whether the rule is paused or not.
    pub is_paused: pulumi_wasm_rust::Output<bool>,
    /// A list of paths to apply the rule to.
    pub paths: pulumi_wasm_rust::Output<Vec<String>>,
    /// The Web Analytics ruleset id. **Modifying this attribute will force creation of a new resource.**
    pub ruleset_id: pulumi_wasm_rust::Output<String>,
}

pub struct WebAnalyticsRuleResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The host to apply the rule to.
    pub host: pulumi_wasm_rust::Output<String>,
    /// Whether the rule includes or excludes the matched traffic from being measured in Web Analytics.
    pub inclusive: pulumi_wasm_rust::Output<bool>,
    /// Whether the rule is paused or not.
    pub is_paused: pulumi_wasm_rust::Output<bool>,
    /// A list of paths to apply the rule to.
    pub paths: pulumi_wasm_rust::Output<Vec<String>>,
    /// The Web Analytics ruleset id. **Modifying this attribute will force creation of a new resource.**
    pub ruleset_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WebAnalyticsRuleArgs) -> WebAnalyticsRuleResult {
    let result = crate::bindings::pulumi::cloudflare::web_analytics_rule::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::web_analytics_rule::Args {
            account_id: &args.account_id.get_inner(),
            host: &args.host.get_inner(),
            inclusive: &args.inclusive.get_inner(),
            is_paused: &args.is_paused.get_inner(),
            paths: &args.paths.get_inner(),
            ruleset_id: &args.ruleset_id.get_inner(),
        },
    );

    WebAnalyticsRuleResult {
        account_id: crate::into_domain(result.account_id),
        host: crate::into_domain(result.host),
        inclusive: crate::into_domain(result.inclusive),
        is_paused: crate::into_domain(result.is_paused),
        paths: crate::into_domain(result.paths),
        ruleset_id: crate::into_domain(result.ruleset_id),
    }
}

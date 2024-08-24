//! Define Firewall rules using filter expressions for more control over
//! how traffic is matched to the rule. A filter expression permits
//! selecting traffic by multiple criteria allowing greater freedom in
//! rule creation.
//! 
//! Filter expressions needs to be created first before using Firewall
//! Rule.
//! 
//! > `cloudflare.FirewallRule` is in a deprecation phase that will last for one
//!   year (May 1st, 2024). During this time period, this resource is still fully
//!   supported but you are strongly advised  to move to the `cloudflare.Ruleset`
//!   resource. Full details can be found in the
//!   developer documentation.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const wordpressFilter = new cloudflare.Filter("wordpressFilter", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     description: "Wordpress break-in attempts that are outside of the office",
//!     expression: "(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1",
//! });
//! const wordpressFirewallRule = new cloudflare.FirewallRule("wordpressFirewallRule", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     description: "Block wordpress break-in attempts",
//!     filterId: wordpressFilter.id,
//!     action: "block",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! wordpress_filter = cloudflare.Filter("wordpressFilter",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     description="Wordpress break-in attempts that are outside of the office",
//!     expression="(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1")
//! wordpress_firewall_rule = cloudflare.FirewallRule("wordpressFirewallRule",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     description="Block wordpress break-in attempts",
//!     filter_id=wordpress_filter.id,
//!     action="block")
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
//!     var wordpressFilter = new Cloudflare.Filter("wordpressFilter", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Description = "Wordpress break-in attempts that are outside of the office",
//!         Expression = "(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1",
//!     });
//! 
//!     var wordpressFirewallRule = new Cloudflare.FirewallRule("wordpressFirewallRule", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Description = "Block wordpress break-in attempts",
//!         FilterId = wordpressFilter.Id,
//!         Action = "block",
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
//! 		wordpressFilter, err := cloudflare.NewFilter(ctx, "wordpressFilter", &cloudflare.FilterArgs{
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Description: pulumi.String("Wordpress break-in attempts that are outside of the office"),
//! 			Expression:  pulumi.String("(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewFirewallRule(ctx, "wordpressFirewallRule", &cloudflare.FirewallRuleArgs{
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Description: pulumi.String("Block wordpress break-in attempts"),
//! 			FilterId:    wordpressFilter.ID(),
//! 			Action:      pulumi.String("block"),
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
//! import com.pulumi.cloudflare.Filter;
//! import com.pulumi.cloudflare.FilterArgs;
//! import com.pulumi.cloudflare.FirewallRule;
//! import com.pulumi.cloudflare.FirewallRuleArgs;
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
//!         var wordpressFilter = new Filter("wordpressFilter", FilterArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .description("Wordpress break-in attempts that are outside of the office")
//!             .expression("(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1")
//!             .build());
//! 
//!         var wordpressFirewallRule = new FirewallRule("wordpressFirewallRule", FirewallRuleArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .description("Block wordpress break-in attempts")
//!             .filterId(wordpressFilter.id())
//!             .action("block")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   wordpressFilter:
//!     type: cloudflare:Filter
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       description: Wordpress break-in attempts that are outside of the office
//!       expression: (http.request.uri.path ~ ".*wp-login.php" or http.request.uri.path ~ ".*xmlrpc.php") and ip.src ne 192.0.2.1
//!   wordpressFirewallRule:
//!     type: cloudflare:FirewallRule
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       description: Block wordpress break-in attempts
//!       filterId: ${wordpressFilter.id}
//!       action: block
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/firewallRule:FirewallRule example <zone_id>/<firewall_rule_id>
//! ```
//! 

pub struct FirewallRuleArgs {
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `allow`, `js_challenge`, `managed_challenge`, `log`, `bypass`.
    pub action: pulumi_wasm_rust::Output<String>,
    /// A description of the rule to help identify it.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The identifier of the Filter to use for determining if the Firewall Rule should be triggered.
    pub filter_id: pulumi_wasm_rust::Output<String>,
    /// Whether this filter based firewall rule is currently paused.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// The priority of the rule to allow control of processing order. A lower number indicates high priority. If not provided, any rules with a priority will be sequenced before those without.
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// List of products to bypass for a request when the bypass action is used. Available values: `zoneLockdown`, `uaBlock`, `bic`, `hot`, `securityLevel`, `rateLimit`, `waf`.
    pub products: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct FirewallRuleResult {
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `allow`, `js_challenge`, `managed_challenge`, `log`, `bypass`.
    pub action: pulumi_wasm_rust::Output<String>,
    /// A description of the rule to help identify it.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The identifier of the Filter to use for determining if the Firewall Rule should be triggered.
    pub filter_id: pulumi_wasm_rust::Output<String>,
    /// Whether this filter based firewall rule is currently paused.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// The priority of the rule to allow control of processing order. A lower number indicates high priority. If not provided, any rules with a priority will be sequenced before those without.
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// List of products to bypass for a request when the bypass action is used. Available values: `zoneLockdown`, `uaBlock`, `bic`, `hot`, `securityLevel`, `rateLimit`, `waf`.
    pub products: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: FirewallRuleArgs) -> FirewallRuleResult {

    let result = crate::bindings::pulumi::cloudflare::firewall_rule::invoke(name, &crate::bindings::pulumi::cloudflare::firewall_rule::Args {
        action: args.action.get_inner(),
        description: args.description.get_inner(),
        filter_id: args.filter_id.get_inner(),
        paused: args.paused.get_inner(),
        priority: args.priority.get_inner(),
        products: args.products.get_inner(),
        zone_id: args.zone_id.get_inner(),
    });

    FirewallRuleResult {
        action: crate::into_domain(result.action),
        description: crate::into_domain(result.description),
        filter_id: crate::into_domain(result.filter_id),
        paused: crate::into_domain(result.paused),
        priority: crate::into_domain(result.priority),
        products: crate::into_domain(result.products),
        zone_id: crate::into_domain(result.zone_id),
    }
}

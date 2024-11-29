//! Magic Firewall is a network-level firewall to protect networks that are onboarded to Cloudflare's Magic Transit. This resource
//! creates a root ruleset on the account level and contains one or more rules. Rules can be crafted in Wireshark syntax and
//! are evaluated in order, with the first rule having the highest priority.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.MagicFirewallRuleset("example", {
//!     accountId: "d41d8cd98f00b204e9800998ecf8427e",
//!     name: "Magic Transit Ruleset",
//!     description: "Global mitigations",
//!     rules: [
//!         {
//!             action: "allow",
//!             expression: "tcp.dstport in { 32768..65535 }",
//!             description: "Allow TCP Ephemeral Ports",
//!             enabled: "true",
//!         },
//!         {
//!             action: "block",
//!             expression: "ip.len >= 0",
//!             description: "Block all",
//!             enabled: "true",
//!         },
//!     ],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.MagicFirewallRuleset("example",
//!     account_id="d41d8cd98f00b204e9800998ecf8427e",
//!     name="Magic Transit Ruleset",
//!     description="Global mitigations",
//!     rules=[
//!         {
//!             "action": "allow",
//!             "expression": "tcp.dstport in { 32768..65535 }",
//!             "description": "Allow TCP Ephemeral Ports",
//!             "enabled": "true",
//!         },
//!         {
//!             "action": "block",
//!             "expression": "ip.len >= 0",
//!             "description": "Block all",
//!             "enabled": "true",
//!         },
//!     ])
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
//!     var example = new Cloudflare.MagicFirewallRuleset("example", new()
//!     {
//!         AccountId = "d41d8cd98f00b204e9800998ecf8427e",
//!         Name = "Magic Transit Ruleset",
//!         Description = "Global mitigations",
//!         Rules = new[]
//!         {
//!             
//!             {
//!                 { "action", "allow" },
//!                 { "expression", "tcp.dstport in { 32768..65535 }" },
//!                 { "description", "Allow TCP Ephemeral Ports" },
//!                 { "enabled", "true" },
//!             },
//!             
//!             {
//!                 { "action", "block" },
//!                 { "expression", "ip.len >= 0" },
//!                 { "description", "Block all" },
//!                 { "enabled", "true" },
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
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := cloudflare.NewMagicFirewallRuleset(ctx, "example", &cloudflare.MagicFirewallRulesetArgs{
//! 			AccountId:   pulumi.String("d41d8cd98f00b204e9800998ecf8427e"),
//! 			Name:        pulumi.String("Magic Transit Ruleset"),
//! 			Description: pulumi.String("Global mitigations"),
//! 			Rules: pulumi.StringMapArray{
//! 				pulumi.StringMap{
//! 					"action":      pulumi.String("allow"),
//! 					"expression":  pulumi.String("tcp.dstport in { 32768..65535 }"),
//! 					"description": pulumi.String("Allow TCP Ephemeral Ports"),
//! 					"enabled":     pulumi.String("true"),
//! 				},
//! 				pulumi.StringMap{
//! 					"action":      pulumi.String("block"),
//! 					"expression":  pulumi.String("ip.len >= 0"),
//! 					"description": pulumi.String("Block all"),
//! 					"enabled":     pulumi.String("true"),
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
//! import com.pulumi.cloudflare.MagicFirewallRuleset;
//! import com.pulumi.cloudflare.MagicFirewallRulesetArgs;
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
//!         var example = new MagicFirewallRuleset("example", MagicFirewallRulesetArgs.builder()
//!             .accountId("d41d8cd98f00b204e9800998ecf8427e")
//!             .name("Magic Transit Ruleset")
//!             .description("Global mitigations")
//!             .rules(            
//!                 Map.ofEntries(
//!                     Map.entry("action", "allow"),
//!                     Map.entry("expression", "tcp.dstport in { 32768..65535 }"),
//!                     Map.entry("description", "Allow TCP Ephemeral Ports"),
//!                     Map.entry("enabled", "true")
//!                 ),
//!                 Map.ofEntries(
//!                     Map.entry("action", "block"),
//!                     Map.entry("expression", "ip.len >= 0"),
//!                     Map.entry("description", "Block all"),
//!                     Map.entry("enabled", "true")
//!                 ))
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:MagicFirewallRuleset
//!     properties:
//!       accountId: d41d8cd98f00b204e9800998ecf8427e
//!       name: Magic Transit Ruleset
//!       description: Global mitigations
//!       rules:
//!         - action: allow
//!           expression: tcp.dstport in { 32768..65535 }
//!           description: Allow TCP Ephemeral Ports
//!           enabled: 'true'
//!         - action: block
//!           expression: ip.len >= 0
//!           description: Block all
//!           enabled: 'true'
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! An existing Magic Firewall Ruleset can be imported using the account ID and ruleset ID
//! 
//! ```sh
//! $ pulumi import cloudflare:index/magicFirewallRuleset:MagicFirewallRuleset example d41d8cd98f00b204e9800998ecf8427e/cb029e245cfdd66dc8d2e570d5dd3322
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct MagicFirewallRulesetArgs {
    /// The ID of the account where the ruleset is being created.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A note that can be used to annotate the rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the ruleset.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub rules: pulumi_wasm_rust::Output<Option<Vec<std::collections::HashMap<String, String>>>>,
}

pub struct MagicFirewallRulesetResult {
    /// The ID of the account where the ruleset is being created.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A note that can be used to annotate the rule.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the ruleset.
    pub name: pulumi_wasm_rust::Output<String>,
    pub rules: pulumi_wasm_rust::Output<Option<Vec<std::collections::HashMap<String, String>>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: MagicFirewallRulesetArgs) -> MagicFirewallRulesetResult {

    let result = crate::bindings::pulumi::cloudflare::magic_firewall_ruleset::invoke(name, &crate::bindings::pulumi::cloudflare::magic_firewall_ruleset::Args {
        account_id: &args.account_id.get_inner(),
        description: &args.description.get_inner(),
        name: &args.name.get_inner(),
        rules: &args.rules.get_inner(),
    });

    MagicFirewallRulesetResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        name: crate::into_domain(result.name),
        rules: crate::into_domain(result.rules),
    }
}

//! Provides a Cloudflare IP Firewall Access Rule resource. Access
//! control can be applied on basis of IP addresses, IP ranges, AS
//! numbers or countries.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! // Challenge requests coming from known Tor exit nodes.
//! const torExitNodes = new cloudflare.AccessRule("torExitNodes", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     notes: "Requests coming from known Tor exit nodes",
//!     mode: "challenge",
//!     configuration: {
//!         target: "country",
//!         value: "T1",
//!     },
//! });
//! // Allowlist requests coming from Antarctica, but only for single zone.
//! const antarctica = new cloudflare.AccessRule("antarctica", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     notes: "Requests coming from Antarctica",
//!     mode: "whitelist",
//!     configuration: {
//!         target: "country",
//!         value: "AQ",
//!     },
//! });
//! const config = new pulumi.Config();
//! const myOffice = config.getObject<Array<string>>("myOffice") || [
//!     "192.0.2.0/24",
//!     "198.51.100.0/24",
//!     "2001:db8::/56",
//! ];
//! const officeNetwork: cloudflare.AccessRule[] = [];
//! for (const range = {value: 0}; range.value < myOffice.length; range.value++) {
//!     officeNetwork.push(new cloudflare.AccessRule(`officeNetwork-${range.value}`, {
//!         accountId: "f037e56e89293a057740de681ac9abbe",
//!         notes: "Requests coming from office network",
//!         mode: "whitelist",
//!         configuration: {
//!             target: "ip_range",
//!             value: myOffice[count.index],
//!         },
//!     }));
//! }
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! # Challenge requests coming from known Tor exit nodes.
//! tor_exit_nodes = cloudflare.AccessRule("torExitNodes",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     notes="Requests coming from known Tor exit nodes",
//!     mode="challenge",
//!     configuration=cloudflare.AccessRuleConfigurationArgs(
//!         target="country",
//!         value="T1",
//!     ))
//! # Allowlist requests coming from Antarctica, but only for single zone.
//! antarctica = cloudflare.AccessRule("antarctica",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     notes="Requests coming from Antarctica",
//!     mode="whitelist",
//!     configuration=cloudflare.AccessRuleConfigurationArgs(
//!         target="country",
//!         value="AQ",
//!     ))
//! config = pulumi.Config()
//! my_office = config.get_object("myOffice")
//! if my_office is None:
//!     my_office = [
//!         "192.0.2.0/24",
//!         "198.51.100.0/24",
//!         "2001:db8::/56",
//!     ]
//! office_network = []
//! for range in [{"value": i} for i in range(0, len(my_office))]:
//!     office_network.append(cloudflare.AccessRule(f"officeNetwork-{range['value']}",
//!         account_id="f037e56e89293a057740de681ac9abbe",
//!         notes="Requests coming from office network",
//!         mode="whitelist",
//!         configuration=cloudflare.AccessRuleConfigurationArgs(
//!             target="ip_range",
//!             value=my_office[count["index"]],
//!         )))
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
//!     // Challenge requests coming from known Tor exit nodes.
//!     var torExitNodes = new Cloudflare.AccessRule("torExitNodes", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Notes = "Requests coming from known Tor exit nodes",
//!         Mode = "challenge",
//!         Configuration = new Cloudflare.Inputs.AccessRuleConfigurationArgs
//!         {
//!             Target = "country",
//!             Value = "T1",
//!         },
//!     });
//!
//!     // Allowlist requests coming from Antarctica, but only for single zone.
//!     var antarctica = new Cloudflare.AccessRule("antarctica", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Notes = "Requests coming from Antarctica",
//!         Mode = "whitelist",
//!         Configuration = new Cloudflare.Inputs.AccessRuleConfigurationArgs
//!         {
//!             Target = "country",
//!             Value = "AQ",
//!         },
//!     });
//!
//!     var config = new Config();
//!     var myOffice = config.GetObject<string[]>("myOffice") ?? new[]
//!     {
//!         "192.0.2.0/24",
//!         "198.51.100.0/24",
//!         "2001:db8::/56",
//!     };
//!     var officeNetwork = new List<Cloudflare.AccessRule>();
//!     for (var rangeIndex = 0; rangeIndex < myOffice.Length; rangeIndex++)
//!     {
//!         var range = new { Value = rangeIndex };
//!         officeNetwork.Add(new Cloudflare.AccessRule($"officeNetwork-{range.Value}", new()
//!         {
//!             AccountId = "f037e56e89293a057740de681ac9abbe",
//!             Notes = "Requests coming from office network",
//!             Mode = "whitelist",
//!             Configuration = new Cloudflare.Inputs.AccessRuleConfigurationArgs
//!             {
//!                 Target = "ip_range",
//!                 Value = myOffice[count.Index],
//!             },
//!         }));
//!     }
//! });
//! ```
//! ### Go
//! ```go
//! package main
//!
//! import (
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
//! )
//!
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		// Challenge requests coming from known Tor exit nodes.
//! 		_, err := cloudflare.NewAccessRule(ctx, "torExitNodes", &cloudflare.AccessRuleArgs{
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Notes:  pulumi.String("Requests coming from known Tor exit nodes"),
//! 			Mode:   pulumi.String("challenge"),
//! 			Configuration: &cloudflare.AccessRuleConfigurationArgs{
//! 				Target: pulumi.String("country"),
//! 				Value:  pulumi.String("T1"),
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Allowlist requests coming from Antarctica, but only for single zone.
//! 		_, err = cloudflare.NewAccessRule(ctx, "antarctica", &cloudflare.AccessRuleArgs{
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Notes:  pulumi.String("Requests coming from Antarctica"),
//! 			Mode:   pulumi.String("whitelist"),
//! 			Configuration: &cloudflare.AccessRuleConfigurationArgs{
//! 				Target: pulumi.String("country"),
//! 				Value:  pulumi.String("AQ"),
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		cfg := config.New(ctx, "")
//! 		myOffice := []string{
//! 			"192.0.2.0/24",
//! 			"198.51.100.0/24",
//! 			"2001:db8::/56",
//! 		}
//! 		if param := cfg.GetObject("myOffice"); param != nil {
//! 			myOffice = param
//! 		}
//! 		var officeNetwork []*cloudflare.AccessRule
//! 		for index := 0; index < len(myOffice); index++ {
//! 			key0 := index
//! 			_ := index
//! 			__res, err := cloudflare.NewAccessRule(ctx, fmt.Sprintf("officeNetwork-%v", key0), &cloudflare.AccessRuleArgs{
//! 				AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 				Notes:     pulumi.String("Requests coming from office network"),
//! 				Mode:      pulumi.String("whitelist"),
//! 				Configuration: &cloudflare.AccessRuleConfigurationArgs{
//! 					Target: pulumi.String("ip_range"),
//! 					Value:  "TODO: call element",
//! 				},
//! 			})
//! 			if err != nil {
//! 				return err
//! 			}
//! 			officeNetwork = append(officeNetwork, __res)
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
//! import com.pulumi.cloudflare.AccessRule;
//! import com.pulumi.cloudflare.AccessRuleArgs;
//! import com.pulumi.cloudflare.inputs.AccessRuleConfigurationArgs;
//! import com.pulumi.codegen.internal.KeyedValue;
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
//!         final var config = ctx.config();
//!         // Challenge requests coming from known Tor exit nodes.
//!         var torExitNodes = new AccessRule("torExitNodes", AccessRuleArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .notes("Requests coming from known Tor exit nodes")
//!             .mode("challenge")
//!             .configuration(AccessRuleConfigurationArgs.builder()
//!                 .target("country")
//!                 .value("T1")
//!                 .build())
//!             .build());
//!
//!         // Allowlist requests coming from Antarctica, but only for single zone.
//!         var antarctica = new AccessRule("antarctica", AccessRuleArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .notes("Requests coming from Antarctica")
//!             .mode("whitelist")
//!             .configuration(AccessRuleConfigurationArgs.builder()
//!                 .target("country")
//!                 .value("AQ")
//!                 .build())
//!             .build());
//!
//!         final var myOffice = config.get("myOffice").orElse(        
//!             "192.0.2.0/24",
//!             "198.51.100.0/24",
//!             "2001:db8::/56");
//!         for (var i = 0; i < myOffice.length(); i++) {
//!             new AccessRule("officeNetwork-" + i, AccessRuleArgs.builder()            
//!                 .accountId("f037e56e89293a057740de681ac9abbe")
//!                 .notes("Requests coming from office network")
//!                 .mode("whitelist")
//!                 .configuration(AccessRuleConfigurationArgs.builder()
//!                     .target("ip_range")
//!                     .value(myOffice[count.index()])
//!                     .build())
//!                 .build());
//!
//!         
//! }
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! configuration:
//!   # Allowlist office's network IP ranges on all account zones (or other lists of
//!   # resources).
//!   myOffice:
//!     type: list(string)
//!     default:
//!       - 192.0.2.0/24
//!       - 198.51.100.0/24
//!       - 2001:db8::/56
//! resources:
//!   # Challenge requests coming from known Tor exit nodes.
//!   torExitNodes:
//!     type: cloudflare:AccessRule
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       notes: Requests coming from known Tor exit nodes
//!       mode: challenge
//!       configuration:
//!         target: country
//!         value: T1
//!   # Allowlist requests coming from Antarctica, but only for single zone.
//!   antarctica:
//!     type: cloudflare:AccessRule
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       notes: Requests coming from Antarctica
//!       mode: whitelist
//!       configuration:
//!         target: country
//!         value: AQ
//!   officeNetwork:
//!     type: cloudflare:AccessRule
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       notes: Requests coming from office network
//!       mode: whitelist
//!       configuration:
//!         target: ip_range
//!         value:
//!           fn::select:
//!             - ${count.index}
//!             - ${myOffice}
//!     options: {}
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! User level access rule import.
//!
//! ```sh
//! $ pulumi import cloudflare:index/accessRule:AccessRule default user/<user_id>/<rule_id>
//! ```
//!
//! Zone level access rule import.
//!
//! ```sh
//! $ pulumi import cloudflare:index/accessRule:AccessRule default zone/<zone_id>/<rule_id>
//! ```
//!
//! Account level access rule import.
//!
//! ```sh
//! $ pulumi import cloudflare:index/accessRule:AccessRule default account/<account_id>/<rule_id>
//! ```
//!

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AccessRuleArgs {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Rule configuration to apply to a matched request. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub configuration: pulumi_wasm_rust::Output<crate::types::AccessRuleConfiguration>,
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`.
    #[builder(into)]
    pub mode: pulumi_wasm_rust::Output<String>,
    /// A personal note about the rule. Typically used as a reminder or explanation for the rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub notes: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessRuleResult {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Rule configuration to apply to a matched request. **Modifying this attribute will force creation of a new resource.**
    pub configuration: pulumi_wasm_rust::Output<crate::types::AccessRuleConfiguration>,
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`.
    pub mode: pulumi_wasm_rust::Output<String>,
    /// A personal note about the rule. Typically used as a reminder or explanation for the rule.
    pub notes: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessRuleArgs) -> AccessRuleResult {
    let result = crate::bindings::pulumi::cloudflare::access_rule::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_rule::Args {
            account_id: &args.account_id.get_inner(),
            configuration: &args.configuration.get_inner(),
            mode: &args.mode.get_inner(),
            notes: &args.notes.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    AccessRuleResult {
        account_id: crate::into_domain(result.account_id),
        configuration: crate::into_domain(result.configuration),
        mode: crate::into_domain(result.mode),
        notes: crate::into_domain(result.notes),
        zone_id: crate::into_domain(result.zone_id),
    }
}

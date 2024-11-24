//! Use this datasource to lookup Rulesets in an account or zone.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getRulesets({
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     filter: {
//!         name: ".*OWASP.*",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_rulesets(zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     filter={
//!         "name": ".*OWASP.*",
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
//!     var example = Cloudflare.GetRulesets.Invoke(new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Filter = new Cloudflare.Inputs.GetRulesetsFilterInputArgs
//!         {
//!             Name = ".*OWASP.*",
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
//! 		_, err := cloudflare.GetRulesets(ctx, &cloudflare.GetRulesetsArgs{
//! 			ZoneId: pulumi.StringRef("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Filter: cloudflare.GetRulesetsFilter{
//! 				Name: pulumi.StringRef(".*OWASP.*"),
//! 			},
//! 		}, nil)
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
//! import com.pulumi.cloudflare.CloudflareFunctions;
//! import com.pulumi.cloudflare.inputs.GetRulesetsArgs;
//! import com.pulumi.cloudflare.inputs.GetRulesetsFilterArgs;
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
//!         final var example = CloudflareFunctions.getRulesets(GetRulesetsArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .filter(GetRulesetsFilterArgs.builder()
//!                 .name(".*OWASP.*")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getRulesets
//!       Arguments:
//!         zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!         filter:
//!           name: .*OWASP.*
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsArgs {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub filter: pulumi_wasm_rust::Output<Option<crate::types::GetRulesetsFilter>>,
    /// Include rule data in response.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub include_rules: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetRulesetsResult {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub filter: pulumi_wasm_rust::Output<Option<crate::types::GetRulesetsFilter>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Include rule data in response.
    pub include_rules: pulumi_wasm_rust::Output<Option<bool>>,
    pub rulesets: pulumi_wasm_rust::Output<Vec<crate::types::GetRulesetsRuleset>>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetRulesetsArgs
) -> GetRulesetsResult {

    let result = crate::bindings::pulumi::cloudflare::get_rulesets::invoke(
        &crate::bindings::pulumi::cloudflare::get_rulesets::Args {
                account_id: &args.account_id.get_inner(),
                filter: &args.filter.get_inner(),
                include_rules: &args.include_rules.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    GetRulesetsResult {
        account_id: crate::into_domain(result.account_id),
        filter: crate::into_domain(result.filter),
        id: crate::into_domain(result.id),
        include_rules: crate::into_domain(result.include_rules),
        rulesets: crate::into_domain(result.rulesets),
        zone_id: crate::into_domain(result.zone_id),
    }
}

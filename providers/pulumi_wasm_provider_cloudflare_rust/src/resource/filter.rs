//! Filter expressions that can be referenced across multiple features,
//! e.g. Firewall Rules. See [what is a filter](https://developers.cloudflare.com/firewall/api/cf-filters/what-is-a-filter/)
//! for more details and available fields and operators.
//!
//! > `cloudflare.Filter` is in a deprecation phase that will last for one
//!   year (May 1st, 2024). During this time period, this resource is still fully
//!   supported but you are strongly advised to move to the
//!   `cloudflare.Ruleset` resource. Full details can be found in the
//!   developer documentation.
//!
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const wordpress = new cloudflare.Filter("wordpress", {
//!     description: "Wordpress break-in attempts that are outside of the office",
//!     expression: "(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! wordpress = cloudflare.Filter("wordpress",
//!     description="Wordpress break-in attempts that are outside of the office",
//!     expression="(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1",
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
//!     var wordpress = new Cloudflare.Filter("wordpress", new()
//!     {
//!         Description = "Wordpress break-in attempts that are outside of the office",
//!         Expression = "(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1",
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
//! 		_, err := cloudflare.NewFilter(ctx, "wordpress", &cloudflare.FilterArgs{
//! 			Description: pulumi.String("Wordpress break-in attempts that are outside of the office"),
//! 			Expression:  pulumi.String("(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1"),
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
//! import com.pulumi.cloudflare.Filter;
//! import com.pulumi.cloudflare.FilterArgs;
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
//!         var wordpress = new Filter("wordpress", FilterArgs.builder()        
//!             .description("Wordpress break-in attempts that are outside of the office")
//!             .expression("(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   wordpress:
//!     type: cloudflare:Filter
//!     properties:
//!       description: Wordpress break-in attempts that are outside of the office
//!       expression: (http.request.uri.path ~ ".*wp-login.php" or http.request.uri.path ~ ".*xmlrpc.php") and ip.src ne 192.0.2.1
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/filter:Filter example <zone_id>/<filter_id>
//! ```
//!

pub struct FilterArgs {
    /// A note that you can use to describe the purpose of the filter.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The filter expression to be used.
    pub expression: pulumi_wasm_rust::Output<String>,
    /// Whether this filter is currently paused.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// Short reference tag to quickly select related rules.
    pub ref_: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct FilterResult {
    /// A note that you can use to describe the purpose of the filter.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The filter expression to be used.
    pub expression: pulumi_wasm_rust::Output<String>,
    /// Whether this filter is currently paused.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// Short reference tag to quickly select related rules.
    pub ref_: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: FilterArgs) -> FilterResult {
    let result = crate::bindings::pulumi::cloudflare::filter::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::filter::Args {
            description: args.description.get_inner(),
            expression: args.expression.get_inner(),
            paused: args.paused.get_inner(),
            ref_: args.ref_.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    FilterResult {
        description: crate::into_domain(result.description),
        expression: crate::into_domain(result.expression),
        paused: crate::into_domain(result.paused),
        ref_: crate::into_domain(result.ref_),
        zone_id: crate::into_domain(result.zone_id),
    }
}

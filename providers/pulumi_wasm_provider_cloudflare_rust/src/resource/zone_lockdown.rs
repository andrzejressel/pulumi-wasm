//! Provides a Cloudflare Zone Lockdown resource. Zone Lockdown allows
//! you to define one or more URLs (with wildcard matching on the domain
//! or path) that will only permit access if the request originates
//! from an IP address that matches a safelist of one or more IP
//! addresses and/or IP ranges.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Restrict access to these endpoints to requests from a known IP address range.
//! const example = new cloudflare.ZoneLockdown("example", {
//!     configurations: [{
//!         target: "ip_range",
//!         value: "192.0.2.0/24",
//!     }],
//!     description: "Restrict access to these endpoints to requests from a known IP address range",
//!     paused: false,
//!     urls: ["api.mysite.com/some/endpoint*"],
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Restrict access to these endpoints to requests from a known IP address range.
//! example = cloudflare.ZoneLockdown("example",
//!     configurations=[cloudflare.ZoneLockdownConfigurationArgs(
//!         target="ip_range",
//!         value="192.0.2.0/24",
//!     )],
//!     description="Restrict access to these endpoints to requests from a known IP address range",
//!     paused=False,
//!     urls=["api.mysite.com/some/endpoint*"],
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
//!     // Restrict access to these endpoints to requests from a known IP address range.
//!     var example = new Cloudflare.ZoneLockdown("example", new()
//!     {
//!         Configurations = new[]
//!         {
//!             new Cloudflare.Inputs.ZoneLockdownConfigurationArgs
//!             {
//!                 Target = "ip_range",
//!                 Value = "192.0.2.0/24",
//!             },
//!         },
//!         Description = "Restrict access to these endpoints to requests from a known IP address range",
//!         Paused = false,
//!         Urls = new[]
//!         {
//!             "api.mysite.com/some/endpoint*",
//!         },
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
//! 		// Restrict access to these endpoints to requests from a known IP address range.
//! 		_, err := cloudflare.NewZoneLockdown(ctx, "example", &cloudflare.ZoneLockdownArgs{
//! 			Configurations: cloudflare.ZoneLockdownConfigurationArray{
//! 				&cloudflare.ZoneLockdownConfigurationArgs{
//! 					Target: pulumi.String("ip_range"),
//! 					Value:  pulumi.String("192.0.2.0/24"),
//! 				},
//! 			},
//! 			Description: pulumi.String("Restrict access to these endpoints to requests from a known IP address range"),
//! 			Paused:      pulumi.Bool(false),
//! 			Urls: pulumi.StringArray{
//! 				pulumi.String("api.mysite.com/some/endpoint*"),
//! 			},
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
//! import com.pulumi.cloudflare.ZoneLockdown;
//! import com.pulumi.cloudflare.ZoneLockdownArgs;
//! import com.pulumi.cloudflare.inputs.ZoneLockdownConfigurationArgs;
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
//!         // Restrict access to these endpoints to requests from a known IP address range.
//!         var example = new ZoneLockdown("example", ZoneLockdownArgs.builder()        
//!             .configurations(ZoneLockdownConfigurationArgs.builder()
//!                 .target("ip_range")
//!                 .value("192.0.2.0/24")
//!                 .build())
//!             .description("Restrict access to these endpoints to requests from a known IP address range")
//!             .paused("false")
//!             .urls("api.mysite.com/some/endpoint*")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Restrict access to these endpoints to requests from a known IP address range.
//!   example:
//!     type: cloudflare:ZoneLockdown
//!     properties:
//!       configurations:
//!         - target: ip_range
//!           value: 192.0.2.0/24
//!       description: Restrict access to these endpoints to requests from a known IP address range
//!       paused: 'false'
//!       urls:
//!         - api.mysite.com/some/endpoint*
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zoneLockdown:ZoneLockdown example <zone_id>/<lockdown_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZoneLockdownArgs {
    /// A list of IP addresses or IP ranges to match the request against specified in target, value pairs.
    #[builder(into)]
    pub configurations: pulumi_wasm_rust::Output<Vec<crate::types::ZoneLockdownConfiguration>>,
    /// A description about the lockdown entry. Typically used as a reminder or explanation for the lockdown.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Boolean of whether this zone lockdown is currently paused. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// A list of simple wildcard patterns to match requests against. The order of the urls is unimportant.
    #[builder(into)]
    pub urls: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneLockdownResult {
    /// A list of IP addresses or IP ranges to match the request against specified in target, value pairs.
    pub configurations: pulumi_wasm_rust::Output<Vec<crate::types::ZoneLockdownConfiguration>>,
    /// A description about the lockdown entry. Typically used as a reminder or explanation for the lockdown.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Boolean of whether this zone lockdown is currently paused. Defaults to `false`.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// A list of simple wildcard patterns to match requests against. The order of the urls is unimportant.
    pub urls: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneLockdownArgs) -> ZoneLockdownResult {

    let result = crate::bindings::pulumi::cloudflare::zone_lockdown::invoke(name, &crate::bindings::pulumi::cloudflare::zone_lockdown::Args {
        configurations: &args.configurations.get_inner(),
        description: &args.description.get_inner(),
        paused: &args.paused.get_inner(),
        priority: &args.priority.get_inner(),
        urls: &args.urls.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZoneLockdownResult {
        configurations: crate::into_domain(result.configurations),
        description: crate::into_domain(result.description),
        paused: crate::into_domain(result.paused),
        priority: crate::into_domain(result.priority),
        urls: crate::into_domain(result.urls),
        zone_id: crate::into_domain(result.zone_id),
    }
}

//! Use this data source to look up Zone results for use in other resources.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getZones({
//!     filter: {
//!         accountId: "f037e56e89293a057740de681ac9abbe",
//!         status: "active",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_zones(filter=cloudflare.GetZonesFilterArgs(
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     status="active",
//! ))
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
//!     var example = Cloudflare.GetZones.Invoke(new()
//!     {
//!         Filter = new Cloudflare.Inputs.GetZonesFilterInputArgs
//!         {
//!             AccountId = "f037e56e89293a057740de681ac9abbe",
//!             Status = "active",
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
//! 		_, err := cloudflare.GetZones(ctx, &cloudflare.GetZonesArgs{
//! 			Filter: cloudflare.GetZonesFilter{
//! 				AccountId: pulumi.StringRef("f037e56e89293a057740de681ac9abbe"),
//! 				Status:    pulumi.StringRef("active"),
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
//! import com.pulumi.cloudflare.inputs.GetZonesArgs;
//! import com.pulumi.cloudflare.inputs.GetZonesFilterArgs;
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
//!         final var example = CloudflareFunctions.getZones(GetZonesArgs.builder()
//!             .filter(GetZonesFilterArgs.builder()
//!                 .accountId("f037e56e89293a057740de681ac9abbe")
//!                 .status("active")
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
//!       Function: cloudflare:getZones
//!       Arguments:
//!         filter:
//!           accountId: f037e56e89293a057740de681ac9abbe
//!           status: active
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetZonesArgs {
    /// One or more values used to look up zone records. If more than one value is given all values must match in order to be included.
    #[builder(into)]
    pub filter: pulumi_wasm_rust::Output<crate::types::GetZonesFilter>,
}

pub struct GetZonesResult {
    /// One or more values used to look up zone records. If more than one value is given all values must match in order to be included.
    pub filter: pulumi_wasm_rust::Output<crate::types::GetZonesFilter>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// A list of zone objects.
    pub zones: pulumi_wasm_rust::Output<Vec<crate::types::GetZonesZone>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetZonesArgs
) -> GetZonesResult {

    let result = crate::bindings::pulumi::cloudflare::get_zones::invoke(
        &crate::bindings::pulumi::cloudflare::get_zones::Args {
                filter: &args.filter.get_inner(),
        }
    );

    GetZonesResult {
        filter: crate::into_domain(result.filter),
        id: crate::into_domain(result.id),
        zones: crate::into_domain(result.zones),
    }
}

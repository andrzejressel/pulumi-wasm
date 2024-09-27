//! Use this data source to look up Zone DNSSEC settings.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getZoneDnssec({
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_zone_dnssec(zone_id="0da42c8d2132a9ddaf714f9e7c920711")
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
//!     var example = Cloudflare.GetZoneDnssec.Invoke(new()
//!     {
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
//! 		_, err := cloudflare.LookupZoneDnssec(ctx, &cloudflare.LookupZoneDnssecArgs{
//! 			ZoneId: "0da42c8d2132a9ddaf714f9e7c920711",
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
//! import com.pulumi.cloudflare.inputs.GetZoneDnssecArgs;
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
//!         final var example = CloudflareFunctions.getZoneDnssec(GetZoneDnssecArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
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
//!       Function: cloudflare:getZoneDnssec
//!       Arguments:
//!         zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetZoneDnssecArgs {
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetZoneDnssecResult {
    /// Zone DNSSEC algorithm.
    pub algorithm: pulumi_wasm_rust::Output<String>,
    /// Zone DNSSEC digest.
    pub digest: pulumi_wasm_rust::Output<String>,
    /// Digest algorithm use for Zone DNSSEC.
    pub digest_algorithm: pulumi_wasm_rust::Output<String>,
    /// Digest Type for Zone DNSSEC.
    pub digest_type: pulumi_wasm_rust::Output<String>,
    /// DS for the Zone DNSSEC.
    pub ds: pulumi_wasm_rust::Output<String>,
    /// Zone DNSSEC flags.
    pub flags: pulumi_wasm_rust::Output<i32>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Key Tag for the Zone DNSSEC.
    pub key_tag: pulumi_wasm_rust::Output<i32>,
    /// Key type used for Zone DNSSEC.
    pub key_type: pulumi_wasm_rust::Output<String>,
    /// Public Key for the Zone DNSSEC.
    pub public_key: pulumi_wasm_rust::Output<String>,
    /// The status of the Zone DNSSEC.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetZoneDnssecArgs) -> GetZoneDnssecResult {

    let result = crate::bindings::pulumi::cloudflare::get_zone_dnssec::invoke(&crate::bindings::pulumi::cloudflare::get_zone_dnssec::Args {
        zone_id: &args.zone_id.get_inner(),
    });

    GetZoneDnssecResult {
        algorithm: crate::into_domain(result.algorithm),
        digest: crate::into_domain(result.digest),
        digest_algorithm: crate::into_domain(result.digest_algorithm),
        digest_type: crate::into_domain(result.digest_type),
        ds: crate::into_domain(result.ds),
        flags: crate::into_domain(result.flags),
        id: crate::into_domain(result.id),
        key_tag: crate::into_domain(result.key_tag),
        key_type: crate::into_domain(result.key_type),
        public_key: crate::into_domain(result.public_key),
        status: crate::into_domain(result.status),
        zone_id: crate::into_domain(result.zone_id),
    }
}

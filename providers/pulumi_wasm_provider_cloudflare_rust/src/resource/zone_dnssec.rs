//! Provides a Cloudflare resource to create and modify zone DNSSEC settings.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.Zone("example", {zone: "example.com"});
//! const exampleZoneDnssec = new cloudflare.ZoneDnssec("example", {zoneId: example.id});
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.Zone("example", zone="example.com")
//! example_zone_dnssec = cloudflare.ZoneDnssec("example", zone_id=example.id)
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
//!     var example = new Cloudflare.Zone("example", new()
//!     {
//!         ZoneName = "example.com",
//!     });
//! 
//!     var exampleZoneDnssec = new Cloudflare.ZoneDnssec("example", new()
//!     {
//!         ZoneId = example.Id,
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
//! 		example, err := cloudflare.NewZone(ctx, "example", &cloudflare.ZoneArgs{
//! 			Zone: pulumi.String("example.com"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewZoneDnssec(ctx, "example", &cloudflare.ZoneDnssecArgs{
//! 			ZoneId: example.ID(),
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
//! import com.pulumi.cloudflare.Zone;
//! import com.pulumi.cloudflare.ZoneArgs;
//! import com.pulumi.cloudflare.ZoneDnssec;
//! import com.pulumi.cloudflare.ZoneDnssecArgs;
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
//!         var example = new Zone("example", ZoneArgs.builder()
//!             .zone("example.com")
//!             .build());
//! 
//!         var exampleZoneDnssec = new ZoneDnssec("exampleZoneDnssec", ZoneDnssecArgs.builder()
//!             .zoneId(example.id())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:Zone
//!     properties:
//!       zone: example.com
//!   exampleZoneDnssec:
//!     type: cloudflare:ZoneDnssec
//!     name: example
//!     properties:
//!       zoneId: ${example.id}
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zoneDnssec:ZoneDnssec example <zone_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZoneDnssecArgs {
    /// Zone DNSSEC updated time.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub modified_on: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneDnssecResult {
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
    /// Key Tag for the Zone DNSSEC.
    pub key_tag: pulumi_wasm_rust::Output<i32>,
    /// Key type used for Zone DNSSEC.
    pub key_type: pulumi_wasm_rust::Output<String>,
    /// Zone DNSSEC updated time.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// Public Key for the Zone DNSSEC.
    pub public_key: pulumi_wasm_rust::Output<String>,
    /// The status of the Zone DNSSEC.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneDnssecArgs) -> ZoneDnssecResult {

    let result = crate::bindings::pulumi::cloudflare::zone_dnssec::invoke(name, &crate::bindings::pulumi::cloudflare::zone_dnssec::Args {
        modified_on: &args.modified_on.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZoneDnssecResult {
        algorithm: crate::into_domain(result.algorithm),
        digest: crate::into_domain(result.digest),
        digest_algorithm: crate::into_domain(result.digest_algorithm),
        digest_type: crate::into_domain(result.digest_type),
        ds: crate::into_domain(result.ds),
        flags: crate::into_domain(result.flags),
        key_tag: crate::into_domain(result.key_tag),
        key_type: crate::into_domain(result.key_type),
        modified_on: crate::into_domain(result.modified_on),
        public_key: crate::into_domain(result.public_key),
        status: crate::into_domain(result.status),
        zone_id: crate::into_domain(result.zone_id),
    }
}

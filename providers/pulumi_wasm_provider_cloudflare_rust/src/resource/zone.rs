//! Provides a Cloudflare Zone resource. Zone is the basic resource for
//! working with Cloudflare and is roughly equivalent to a domain name
//! that the user purchases.
//! 
//! > If you are attempting to sign up a subdomain of a zone you must first have Subdomain Support entitlement for your account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.Zone("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     zone: "example.com",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.Zone("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     zone="example.com")
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
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         ZoneName = "example.com",
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
//! 		_, err := cloudflare.NewZone(ctx, "example", &cloudflare.ZoneArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Zone:      pulumi.String("example.com"),
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
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .zone("example.com")
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
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       zone: example.com
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zone:Zone example <zone_id>
//! ```
//! 

pub struct ZoneArgs {
    /// Account ID to manage the zone resource in.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to scan for DNS records on creation. Ignored after zone is created.
    pub jump_start: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether this zone is paused (traffic bypasses Cloudflare). Defaults to `false`.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the commercial plan to apply to the zone. Available values: `free`, `lite`, `pro`, `pro_plus`, `business`, `enterprise`, `partners_free`, `partners_pro`, `partners_business`, `partners_enterprise`.
    pub plan: pulumi_wasm_rust::Output<Option<String>>,
    /// A full zone implies that DNS is hosted with Cloudflare. A partial zone is typically a partner-hosted zone or a CNAME setup. Available values: `full`, `partial`, `secondary`. Defaults to `full`.
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
    /// The DNS zone name which will be added. **Modifying this attribute will force creation of a new resource.**
    pub zone: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneResult {
    /// Account ID to manage the zone resource in.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether to scan for DNS records on creation. Ignored after zone is created.
    pub jump_start: pulumi_wasm_rust::Output<Option<bool>>,
    pub meta: pulumi_wasm_rust::Output<std::collections::HashMap<String, bool>>,
    /// Cloudflare-assigned name servers. This is only populated for zones that use Cloudflare DNS.
    pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    /// Whether this zone is paused (traffic bypasses Cloudflare). Defaults to `false`.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the commercial plan to apply to the zone. Available values: `free`, `lite`, `pro`, `pro_plus`, `business`, `enterprise`, `partners_free`, `partners_pro`, `partners_business`, `partners_enterprise`.
    pub plan: pulumi_wasm_rust::Output<String>,
    /// Status of the zone. Available values: `active`, `pending`, `initializing`, `moved`, `deleted`, `deactivated`.
    pub status: pulumi_wasm_rust::Output<String>,
    /// A full zone implies that DNS is hosted with Cloudflare. A partial zone is typically a partner-hosted zone or a CNAME setup. Available values: `full`, `partial`, `secondary`. Defaults to `full`.
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
    /// List of Vanity Nameservers (if set).
    pub vanity_name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    /// Contains the TXT record value to validate domain ownership. This is only populated for zones of type `partial`.
    pub verification_key: pulumi_wasm_rust::Output<String>,
    /// The DNS zone name which will be added. **Modifying this attribute will force creation of a new resource.**
    pub zone: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneArgs) -> ZoneResult {

    let result = crate::bindings::pulumi::cloudflare::zone::invoke(name, &crate::bindings::pulumi::cloudflare::zone::Args {
        account_id: args.account_id.get_inner(),
        jump_start: args.jump_start.get_inner(),
        paused: args.paused.get_inner(),
        plan: args.plan.get_inner(),
        type_: args.type_.get_inner(),
        zone: args.zone.get_inner(),
    });

    ZoneResult {
        account_id: crate::into_domain(result.account_id),
        jump_start: crate::into_domain(result.jump_start),
        meta: crate::into_domain(result.meta),
        name_servers: crate::into_domain(result.name_servers),
        paused: crate::into_domain(result.paused),
        plan: crate::into_domain(result.plan),
        status: crate::into_domain(result.status),
        type_: crate::into_domain(result.type_),
        vanity_name_servers: crate::into_domain(result.vanity_name_servers),
        verification_key: crate::into_domain(result.verification_key),
        zone: crate::into_domain(result.zone),
    }
}

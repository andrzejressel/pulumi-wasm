//! Use this data source to look up [zone](https://api.cloudflare.com/#zone-properties)
//! info. This is the singular alternative to `cloudflare.getZones`.
//! 
//! > **Note** Cloudflare zone names **are not unique**. It is possible for multiple
//! accounts to have the same zone created but in different states. If you are
//! using this setup, it is advised to use the `account_id` attribute on this
//! resource or swap to `cloudflare.getZones` to further filter the results.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getZone({
//!     name: "example.com",
//! });
//! const exampleRecord = new cloudflare.Record("example", {
//!     zoneId: example.then(example => example.id),
//!     name: "www",
//!     content: "203.0.113.1",
//!     type: "A",
//!     proxied: true,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_zone(name="example.com")
//! example_record = cloudflare.Record("example",
//!     zone_id=example.id,
//!     name="www",
//!     content="203.0.113.1",
//!     type="A",
//!     proxied=True)
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
//!     var example = Cloudflare.GetZone.Invoke(new()
//!     {
//!         Name = "example.com",
//!     });
//! 
//!     var exampleRecord = new Cloudflare.Record("example", new()
//!     {
//!         ZoneId = example.Apply(getZoneResult => getZoneResult.Id),
//!         Name = "www",
//!         Content = "203.0.113.1",
//!         Type = "A",
//!         Proxied = true,
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
//! 		example, err := cloudflare.LookupZone(ctx, &cloudflare.LookupZoneArgs{
//! 			Name: pulumi.StringRef("example.com"),
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewRecord(ctx, "example", &cloudflare.RecordArgs{
//! 			ZoneId:  pulumi.String(example.Id),
//! 			Name:    pulumi.String("www"),
//! 			Content: pulumi.String("203.0.113.1"),
//! 			Type:    pulumi.String("A"),
//! 			Proxied: pulumi.Bool(true),
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
//! import com.pulumi.cloudflare.CloudflareFunctions;
//! import com.pulumi.cloudflare.inputs.GetZoneArgs;
//! import com.pulumi.cloudflare.Record;
//! import com.pulumi.cloudflare.RecordArgs;
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
//!         final var example = CloudflareFunctions.getZone(GetZoneArgs.builder()
//!             .name("example.com")
//!             .build());
//! 
//!         var exampleRecord = new Record("exampleRecord", RecordArgs.builder()
//!             .zoneId(example.applyValue(getZoneResult -> getZoneResult.id()))
//!             .name("www")
//!             .content("203.0.113.1")
//!             .type("A")
//!             .proxied(true)
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   exampleRecord:
//!     type: cloudflare:Record
//!     name: example
//!     properties:
//!       zoneId: ${example.id}
//!       name: www
//!       content: 203.0.113.1
//!       type: A
//!       proxied: true
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getZone
//!       Arguments:
//!         name: example.com
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetZoneArgs {
    /// The account identifier to target for the resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the zone. Must provide only one of `zone_id`, `name`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `name`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetZoneResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The name of the zone. Must provide only one of `zone_id`, `name`.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Cloudflare assigned name servers. This is only populated for zones that use Cloudflare DNS.
    pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    /// Whether the zone is paused on Cloudflare.
    pub paused: pulumi_wasm_rust::Output<bool>,
    /// The name of the plan associated with the zone.
    pub plan: pulumi_wasm_rust::Output<String>,
    /// Status of the zone.
    pub status: pulumi_wasm_rust::Output<String>,
    /// List of Vanity Nameservers (if set).
    pub vanity_name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `name`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetZoneArgs
) -> GetZoneResult {

    let result = crate::bindings::pulumi::cloudflare::get_zone::invoke(
        &crate::bindings::pulumi::cloudflare::get_zone::Args {
                account_id: &args.account_id.get_inner(),
                name: &args.name.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    GetZoneResult {
        account_id: crate::into_domain(result.account_id),
        id: crate::into_domain(result.id),
        name: crate::into_domain(result.name),
        name_servers: crate::into_domain(result.name_servers),
        paused: crate::into_domain(result.paused),
        plan: crate::into_domain(result.plan),
        status: crate::into_domain(result.status),
        vanity_name_servers: crate::into_domain(result.vanity_name_servers),
        zone_id: crate::into_domain(result.zone_id),
    }
}

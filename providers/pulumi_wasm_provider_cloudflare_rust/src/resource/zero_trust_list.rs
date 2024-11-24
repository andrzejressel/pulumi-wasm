//! Provides a Cloudflare Teams List resource. Teams lists are
//! referenced when creating secure web gateway policies or device
//! posture rules.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.ZeroTrustList("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "Corporate devices",
//!     type: "SERIAL",
//!     description: "Serial numbers for all corporate devices.",
//!     items: [
//!         "8GE8721REF",
//!         "5RE8543EGG",
//!         "1YE2880LNP",
//!     ],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.ZeroTrustList("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="Corporate devices",
//!     type="SERIAL",
//!     description="Serial numbers for all corporate devices.",
//!     items=[
//!         "8GE8721REF",
//!         "5RE8543EGG",
//!         "1YE2880LNP",
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
//!     var example = new Cloudflare.ZeroTrustList("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "Corporate devices",
//!         Type = "SERIAL",
//!         Description = "Serial numbers for all corporate devices.",
//!         Items = new[]
//!         {
//!             "8GE8721REF",
//!             "5RE8543EGG",
//!             "1YE2880LNP",
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
//! 		_, err := cloudflare.NewZeroTrustList(ctx, "example", &cloudflare.ZeroTrustListArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:        pulumi.String("Corporate devices"),
//! 			Type:        pulumi.String("SERIAL"),
//! 			Description: pulumi.String("Serial numbers for all corporate devices."),
//! 			Items: pulumi.StringArray{
//! 				pulumi.String("8GE8721REF"),
//! 				pulumi.String("5RE8543EGG"),
//! 				pulumi.String("1YE2880LNP"),
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
//! import com.pulumi.cloudflare.ZeroTrustList;
//! import com.pulumi.cloudflare.ZeroTrustListArgs;
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
//!         var example = new ZeroTrustList("example", ZeroTrustListArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("Corporate devices")
//!             .type("SERIAL")
//!             .description("Serial numbers for all corporate devices.")
//!             .items(            
//!                 "8GE8721REF",
//!                 "5RE8543EGG",
//!                 "1YE2880LNP")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:ZeroTrustList
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: Corporate devices
//!       type: SERIAL
//!       description: Serial numbers for all corporate devices.
//!       items:
//!         - 8GE8721REF
//!         - 5RE8543EGG
//!         - 1YE2880LNP
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustList:ZeroTrustList example <account_id>/<teams_list_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustListArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The description of the teams list.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The items of the teams list.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub items: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The items of the teams list that has explicit description.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub items_with_descriptions: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustListItemsWithDescription>>>,
    /// Name of the teams list.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The teams list type. Available values: `IP`, `SERIAL`, `URL`, `DOMAIN`, `EMAIL`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct ZeroTrustListResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The description of the teams list.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The items of the teams list.
    pub items: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The items of the teams list that has explicit description.
    pub items_with_descriptions: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustListItemsWithDescription>>>,
    /// Name of the teams list.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The teams list type. Available values: `IP`, `SERIAL`, `URL`, `DOMAIN`, `EMAIL`.
    pub type_: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustListArgs) -> ZeroTrustListResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_list::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_list::Args {
        account_id: &args.account_id.get_inner(),
        description: &args.description.get_inner(),
        items: &args.items.get_inner(),
        items_with_descriptions: &args.items_with_descriptions.get_inner(),
        name: &args.name.get_inner(),
        type_: &args.type_.get_inner(),
    });

    ZeroTrustListResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        items: crate::into_domain(result.items),
        items_with_descriptions: crate::into_domain(result.items_with_descriptions),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}

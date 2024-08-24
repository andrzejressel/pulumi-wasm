//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.R2Bucket("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     location: "enam",
//!     name: "terraform-bucket",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.R2Bucket("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     location="enam",
//!     name="terraform-bucket")
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
//!     var example = new Cloudflare.R2Bucket("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Location = "enam",
//!         Name = "terraform-bucket",
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
//! 		_, err := cloudflare.NewR2Bucket(ctx, "example", &cloudflare.R2BucketArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Location:  pulumi.String("enam"),
//! 			Name:      pulumi.String("terraform-bucket"),
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
//! import com.pulumi.cloudflare.R2Bucket;
//! import com.pulumi.cloudflare.R2BucketArgs;
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
//!         var example = new R2Bucket("example", R2BucketArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .location("enam")
//!             .name("terraform-bucket")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:R2Bucket
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       location: enam
//!       name: terraform-bucket
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! > Available location values can be found in the [R2 documentation](https://developers.cloudflare.com/r2/reference/data-location/#available-hints).
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/r2Bucket:R2Bucket default <account id>/<bucket name>
//! ```
//!

pub struct R2BucketArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The location hint of the R2 bucket.
    pub location: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the R2 bucket.
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct R2BucketResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The location hint of the R2 bucket.
    pub location: pulumi_wasm_rust::Output<String>,
    /// The name of the R2 bucket.
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: R2BucketArgs) -> R2BucketResult {
    let result = crate::bindings::pulumi::cloudflare::r2_bucket::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::r2_bucket::Args {
            account_id: &args.account_id.get_inner(),
            location: &args.location.get_inner(),
            name: &args.name.get_inner(),
        },
    );

    R2BucketResult {
        account_id: crate::into_domain(result.account_id),
        location: crate::into_domain(result.location),
        name: crate::into_domain(result.name),
    }
}

//! The [D1 Database](https://developers.cloudflare.com/d1/) resource allows you to manage Cloudflare D1 databases.
//! 
//! !> When a D1 Database is replaced all the data is lost. Please ensure you have a
//!    backup of your data before replacing a D1 Database.
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
//! const example = new cloudflare.D1Database("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "terraform-database",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.D1Database("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="terraform-database")
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
//!     var example = new Cloudflare.D1Database("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "terraform-database",
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
//! 		_, err := cloudflare.NewD1Database(ctx, "example", &cloudflare.D1DatabaseArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("terraform-database"),
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
//! import com.pulumi.cloudflare.D1Database;
//! import com.pulumi.cloudflare.D1DatabaseArgs;
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
//!         var example = new D1Database("example", D1DatabaseArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("terraform-database")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:D1Database
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: terraform-database
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/d1Database:D1Database example <account id>/<database id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct D1DatabaseArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the D1 Database.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct D1DatabaseResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the D1 Database.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The backend version of the database.
    pub version: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: D1DatabaseArgs) -> D1DatabaseResult {

    let result = crate::bindings::pulumi::cloudflare::d1_database::invoke(name, &crate::bindings::pulumi::cloudflare::d1_database::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
    });

    D1DatabaseResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
        version: crate::into_domain(result.version),
    }
}

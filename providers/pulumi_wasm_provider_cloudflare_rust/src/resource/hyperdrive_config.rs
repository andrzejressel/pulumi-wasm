//! The [Hyperdrive Config](https://developers.cloudflare.com/hyperdrive/) resource allows you to manage Cloudflare Hyperdrive Configs.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const noDefaults = new cloudflare.HyperdriveConfig("noDefaults", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "my-hyperdrive-config",
//!     origin: {
//!         database: "postgres",
//!         host: "my-database.example.com",
//!         password: "my-password",
//!         port: 5432,
//!         scheme: "postgres",
//!         user: "my-user",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! no_defaults = cloudflare.HyperdriveConfig("noDefaults",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="my-hyperdrive-config",
//!     origin=cloudflare.HyperdriveConfigOriginArgs(
//!         database="postgres",
//!         host="my-database.example.com",
//!         password="my-password",
//!         port=5432,
//!         scheme="postgres",
//!         user="my-user",
//!     ))
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
//!     var noDefaults = new Cloudflare.HyperdriveConfig("noDefaults", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "my-hyperdrive-config",
//!         Origin = new Cloudflare.Inputs.HyperdriveConfigOriginArgs
//!         {
//!             Database = "postgres",
//!             Host = "my-database.example.com",
//!             Password = "my-password",
//!             Port = 5432,
//!             Scheme = "postgres",
//!             User = "my-user",
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
//! 		_, err := cloudflare.NewHyperdriveConfig(ctx, "noDefaults", &cloudflare.HyperdriveConfigArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("my-hyperdrive-config"),
//! 			Origin: &cloudflare.HyperdriveConfigOriginArgs{
//! 				Database: pulumi.String("postgres"),
//! 				Host:     pulumi.String("my-database.example.com"),
//! 				Password: pulumi.String("my-password"),
//! 				Port:     pulumi.Int(5432),
//! 				Scheme:   pulumi.String("postgres"),
//! 				User:     pulumi.String("my-user"),
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
//! import com.pulumi.cloudflare.HyperdriveConfig;
//! import com.pulumi.cloudflare.HyperdriveConfigArgs;
//! import com.pulumi.cloudflare.inputs.HyperdriveConfigOriginArgs;
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
//!         var noDefaults = new HyperdriveConfig("noDefaults", HyperdriveConfigArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("my-hyperdrive-config")
//!             .origin(HyperdriveConfigOriginArgs.builder()
//!                 .database("postgres")
//!                 .host("my-database.example.com")
//!                 .password("my-password")
//!                 .port(5432)
//!                 .scheme("postgres")
//!                 .user("my-user")
//!                 .build())
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   noDefaults:
//!     type: cloudflare:HyperdriveConfig
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: my-hyperdrive-config
//!       origin:
//!         database: postgres
//!         host: my-database.example.com
//!         password: my-password
//!         port: 5432
//!         scheme: postgres
//!         user: my-user
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/hyperdriveConfig:HyperdriveConfig example <account_id>/<hyperdrive_config_id>
//! ```
//!

pub struct HyperdriveConfigArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The caching details for the Hyperdrive configuration.
    pub caching: pulumi_wasm_rust::Output<Option<crate::types::HyperdriveConfigCaching>>,
    /// The name of the Hyperdrive configuration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The origin details for the Hyperdrive configuration.
    pub origin: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigOrigin>,
}

pub struct HyperdriveConfigResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The caching details for the Hyperdrive configuration.
    pub caching: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigCaching>,
    /// The name of the Hyperdrive configuration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The origin details for the Hyperdrive configuration.
    pub origin: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigOrigin>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: HyperdriveConfigArgs) -> HyperdriveConfigResult {
    let result = crate::bindings::pulumi::cloudflare::hyperdrive_config::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::hyperdrive_config::Args {
            account_id: args.account_id.get_inner(),
            caching: args.caching.get_inner(),
            name: args.name.get_inner(),
            origin: args.origin.get_inner(),
        },
    );

    HyperdriveConfigResult {
        account_id: crate::into_domain(result.account_id),
        caching: crate::into_domain(result.caching),
        name: crate::into_domain(result.name),
        origin: crate::into_domain(result.origin),
    }
}

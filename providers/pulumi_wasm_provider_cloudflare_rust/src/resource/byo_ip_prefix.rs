//! Provides the ability to manage Bring-Your-Own-IP prefixes (BYOIP)
//! which are used with or without Magic Transit.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.ByoIpPrefix("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     advertisement: "on",
//!     description: "Example IP Prefix",
//!     prefixId: "d41d8cd98f00b204e9800998ecf8427e",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.ByoIpPrefix("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     advertisement="on",
//!     description="Example IP Prefix",
//!     prefix_id="d41d8cd98f00b204e9800998ecf8427e")
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
//!     var example = new Cloudflare.ByoIpPrefix("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Advertisement = "on",
//!         Description = "Example IP Prefix",
//!         PrefixId = "d41d8cd98f00b204e9800998ecf8427e",
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
//! 		_, err := cloudflare.NewByoIpPrefix(ctx, "example", &cloudflare.ByoIpPrefixArgs{
//! 			AccountId:     pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Advertisement: pulumi.String("on"),
//! 			Description:   pulumi.String("Example IP Prefix"),
//! 			PrefixId:      pulumi.String("d41d8cd98f00b204e9800998ecf8427e"),
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
//! import com.pulumi.cloudflare.ByoIpPrefix;
//! import com.pulumi.cloudflare.ByoIpPrefixArgs;
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
//!         var example = new ByoIpPrefix("example", ByoIpPrefixArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .advertisement("on")
//!             .description("Example IP Prefix")
//!             .prefixId("d41d8cd98f00b204e9800998ecf8427e")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:ByoIpPrefix
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       advertisement: on
//!       description: Example IP Prefix
//!       prefixId: d41d8cd98f00b204e9800998ecf8427e
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/byoIpPrefix:ByoIpPrefix example <account_id>/<prefix_id>
//! ```
//!

pub struct ByoIpPrefixArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether or not the prefix shall be announced. A prefix can be activated or deactivated once every 15 minutes (attempting more regular updates will trigger rate limiting). Available values: `on`, `off`.
    pub advertisement: pulumi_wasm_rust::Output<Option<String>>,
    /// Description of the BYO IP prefix.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The assigned Bring-Your-Own-IP prefix ID. **Modifying this attribute will force creation of a new resource.**
    pub prefix_id: pulumi_wasm_rust::Output<String>,
}

pub struct ByoIpPrefixResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether or not the prefix shall be announced. A prefix can be activated or deactivated once every 15 minutes (attempting more regular updates will trigger rate limiting). Available values: `on`, `off`.
    pub advertisement: pulumi_wasm_rust::Output<String>,
    /// Description of the BYO IP prefix.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The assigned Bring-Your-Own-IP prefix ID. **Modifying this attribute will force creation of a new resource.**
    pub prefix_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ByoIpPrefixArgs) -> ByoIpPrefixResult {
    let result = crate::bindings::pulumi::cloudflare::byo_ip_prefix::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::byo_ip_prefix::Args {
            account_id: &args.account_id.get_inner(),
            advertisement: &args.advertisement.get_inner(),
            description: &args.description.get_inner(),
            prefix_id: &args.prefix_id.get_inner(),
        },
    );

    ByoIpPrefixResult {
        account_id: crate::into_domain(result.account_id),
        advertisement: crate::into_domain(result.advertisement),
        description: crate::into_domain(result.description),
        prefix_id: crate::into_domain(result.prefix_id),
    }
}

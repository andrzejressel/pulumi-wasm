//! The [Email Routing Address](https://developers.cloudflare.com/email-routing/setup/email-routing-addresses/#destination-addresses) resource allows you to manage Cloudflare Email Routing Destination Addresses.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.EmailRoutingAddress("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     email: "user@example.com",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.EmailRoutingAddress("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     email="user@example.com")
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
//!     var example = new Cloudflare.EmailRoutingAddress("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Email = "user@example.com",
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
//! 		_, err := cloudflare.NewEmailRoutingAddress(ctx, "example", &cloudflare.EmailRoutingAddressArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Email:     pulumi.String("user@example.com"),
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
//! import com.pulumi.cloudflare.EmailRoutingAddress;
//! import com.pulumi.cloudflare.EmailRoutingAddressArgs;
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
//!         var example = new EmailRoutingAddress("example", EmailRoutingAddressArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .email("user@example.com")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:EmailRoutingAddress
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       email: user@example.com
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/emailRoutingAddress:EmailRoutingAddress example <account_id>/<email_routing_id>
//! ```
//! 

pub struct EmailRoutingAddressArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The contact email address of the user.
    pub email: pulumi_wasm_rust::Output<String>,
}

pub struct EmailRoutingAddressResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The date and time the destination address has been created.
    pub created: pulumi_wasm_rust::Output<String>,
    /// The contact email address of the user.
    pub email: pulumi_wasm_rust::Output<String>,
    /// The date and time the destination address has been modified.
    pub modified: pulumi_wasm_rust::Output<String>,
    /// Destination address identifier.
    pub tag: pulumi_wasm_rust::Output<String>,
    /// The date and time the destination address has been verified. Null means not verified yet.
    pub verified: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: EmailRoutingAddressArgs) -> EmailRoutingAddressResult {

    let result = crate::bindings::pulumi::cloudflare::email_routing_address::invoke(name, &crate::bindings::pulumi::cloudflare::email_routing_address::Args {
        account_id: args.account_id.get_inner(),
        email: args.email.get_inner(),
    });

    EmailRoutingAddressResult {
        account_id: crate::into_domain(result.account_id),
        created: crate::into_domain(result.created),
        email: crate::into_domain(result.email),
        modified: crate::into_domain(result.modified),
        tag: crate::into_domain(result.tag),
        verified: crate::into_domain(result.verified),
    }
}

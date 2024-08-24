//! Provides a Cloudflare Account resource. Account is the basic resource for
//! working with Cloudflare zones, teams and users.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.Account("example", {
//!     enforceTwofactor: true,
//!     name: "some-enterprise-account",
//!     type: "enterprise",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.Account("example",
//!     enforce_twofactor=True,
//!     name="some-enterprise-account",
//!     type="enterprise")
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
//!     var example = new Cloudflare.Account("example", new()
//!     {
//!         EnforceTwofactor = true,
//!         Name = "some-enterprise-account",
//!         Type = "enterprise",
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
//! 		_, err := cloudflare.NewAccount(ctx, "example", &cloudflare.AccountArgs{
//! 			EnforceTwofactor: pulumi.Bool(true),
//! 			Name:             pulumi.String("some-enterprise-account"),
//! 			Type:             pulumi.String("enterprise"),
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
//! import com.pulumi.cloudflare.Account;
//! import com.pulumi.cloudflare.AccountArgs;
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
//!         var example = new Account("example", AccountArgs.builder()        
//!             .enforceTwofactor(true)
//!             .name("some-enterprise-account")
//!             .type("enterprise")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:Account
//!     properties:
//!       enforceTwofactor: true
//!       name: some-enterprise-account
//!       type: enterprise
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/account:Account example <account_id>
//! ```
//!

pub struct AccountArgs {
    /// Whether 2FA is enforced on the account. Defaults to `false`.
    pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the account that is displayed in the Cloudflare dashboard.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Account type. Available values: `enterprise`, `standard`. Defaults to `standard`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccountResult {
    /// Whether 2FA is enforced on the account. Defaults to `false`.
    pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the account that is displayed in the Cloudflare dashboard.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Account type. Available values: `enterprise`, `standard`. Defaults to `standard`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccountArgs) -> AccountResult {
    let result = crate::bindings::pulumi::cloudflare::account::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::account::Args {
            enforce_twofactor: &args.enforce_twofactor.get_inner(),
            name: &args.name.get_inner(),
            type_: &args.type_.get_inner(),
        },
    );

    AccountResult {
        enforce_twofactor: crate::into_domain(result.enforce_twofactor),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}

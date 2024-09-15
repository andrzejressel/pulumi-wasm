//! Access Service Tokens are used for service-to-service communication
//! when an application is behind Cloudflare Access.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! // Generate a service token that will renew if terraform is ran within 30 days of expiration
//! const myApp = new cloudflare.AccessServiceToken("myApp", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     minDaysForRenewal: 30,
//!     name: "CI/CD app renewed",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! # Generate a service token that will renew if terraform is ran within 30 days of expiration
//! my_app = cloudflare.AccessServiceToken("myApp",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     min_days_for_renewal=30,
//!     name="CI/CD app renewed")
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
//!     // Generate a service token that will renew if terraform is ran within 30 days of expiration
//!     var myApp = new Cloudflare.AccessServiceToken("myApp", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         MinDaysForRenewal = 30,
//!         Name = "CI/CD app renewed",
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
//! 		// Generate a service token that will renew if terraform is ran within 30 days of expiration
//! 		_, err := cloudflare.NewAccessServiceToken(ctx, "myApp", &cloudflare.AccessServiceTokenArgs{
//! 			AccountId:         pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			MinDaysForRenewal: pulumi.Int(30),
//! 			Name:              pulumi.String("CI/CD app renewed"),
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
//! import com.pulumi.cloudflare.AccessServiceToken;
//! import com.pulumi.cloudflare.AccessServiceTokenArgs;
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
//!         // Generate a service token that will renew if terraform is ran within 30 days of expiration
//!         var myApp = new AccessServiceToken("myApp", AccessServiceTokenArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .minDaysForRenewal(30)
//!             .name("CI/CD app renewed")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Generate a service token that will renew if terraform is ran within 30 days of expiration
//!   myApp:
//!     type: cloudflare:AccessServiceToken
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       minDaysForRenewal: 30
//!       name: CI/CD app renewed
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! If you are importing an Access Service Token you will not have the
//!
//! client_secret available in the state for use. The client_secret is only
//!
//! available once, at creation. In most cases, it is better to just create a new
//!
//! resource should you need to reference it in other resources.
//!
//! ```sh
//! $ pulumi import cloudflare:index/accessServiceToken:AccessServiceToken example <account_id>/<service_token_id>
//! ```
//!

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AccessServiceTokenArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Length of time the service token is valid for. Available values: `8760h`, `17520h`, `43800h`, `87600h`, `forever`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub duration: pulumi_wasm_rust::Output<Option<String>>,
    /// Refresh the token if terraform is run within the specified amount of days before expiration.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    /// Friendly name of the token's intent.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessServiceTokenResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Client ID associated with the Service Token. **Modifying this attribute will force creation of a new resource.**
    pub client_id: pulumi_wasm_rust::Output<String>,
    /// A secret for interacting with Access protocols. **Modifying this attribute will force creation of a new resource.**
    pub client_secret: pulumi_wasm_rust::Output<String>,
    /// Length of time the service token is valid for. Available values: `8760h`, `17520h`, `43800h`, `87600h`, `forever`.
    pub duration: pulumi_wasm_rust::Output<String>,
    /// Date when the token expires.
    pub expires_at: pulumi_wasm_rust::Output<String>,
    /// Refresh the token if terraform is run within the specified amount of days before expiration.
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    /// Friendly name of the token's intent.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessServiceTokenArgs) -> AccessServiceTokenResult {
    let result = crate::bindings::pulumi::cloudflare::access_service_token::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_service_token::Args {
            account_id: &args.account_id.get_inner(),
            duration: &args.duration.get_inner(),
            min_days_for_renewal: &args.min_days_for_renewal.get_inner(),
            name: &args.name.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    AccessServiceTokenResult {
        account_id: crate::into_domain(result.account_id),
        client_id: crate::into_domain(result.client_id),
        client_secret: crate::into_domain(result.client_secret),
        duration: crate::into_domain(result.duration),
        expires_at: crate::into_domain(result.expires_at),
        min_days_for_renewal: crate::into_domain(result.min_days_for_renewal),
        name: crate::into_domain(result.name),
        zone_id: crate::into_domain(result.zone_id),
    }
}

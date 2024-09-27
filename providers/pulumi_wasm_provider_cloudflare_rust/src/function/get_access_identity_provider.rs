//! Use this data source to lookup a single [Access Identity Provider](https://developers.cloudflare.com/cloudflare-one/identity/idp-integration) by name.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const exampleAccessIdentityProvider = cloudflare.getAccessIdentityProvider({
//!     name: "Google SSO",
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//! });
//! const exampleAccessApplication = new cloudflare.AccessApplication("exampleAccessApplication", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     name: "name",
//!     domain: "name.example.com",
//!     type: "self_hosted",
//!     sessionDuration: "24h",
//!     allowedIdps: [exampleAccessIdentityProvider.then(exampleAccessIdentityProvider => exampleAccessIdentityProvider.id)],
//!     autoRedirectToIdentity: true,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example_access_identity_provider = cloudflare.get_access_identity_provider(name="Google SSO",
//!     account_id="f037e56e89293a057740de681ac9abbe")
//! example_access_application = cloudflare.AccessApplication("exampleAccessApplication",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     name="name",
//!     domain="name.example.com",
//!     type="self_hosted",
//!     session_duration="24h",
//!     allowed_idps=[example_access_identity_provider.id],
//!     auto_redirect_to_identity=True)
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
//!     var exampleAccessIdentityProvider = Cloudflare.GetAccessIdentityProvider.Invoke(new()
//!     {
//!         Name = "Google SSO",
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!     });
//! 
//!     var exampleAccessApplication = new Cloudflare.AccessApplication("exampleAccessApplication", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Name = "name",
//!         Domain = "name.example.com",
//!         Type = "self_hosted",
//!         SessionDuration = "24h",
//!         AllowedIdps = new[]
//!         {
//!             exampleAccessIdentityProvider.Apply(getAccessIdentityProviderResult => getAccessIdentityProviderResult.Id),
//!         },
//!         AutoRedirectToIdentity = true,
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
//! 		exampleAccessIdentityProvider, err := cloudflare.LookupAccessIdentityProvider(ctx, &cloudflare.LookupAccessIdentityProviderArgs{
//! 			Name:      "Google SSO",
//! 			AccountId: pulumi.StringRef("f037e56e89293a057740de681ac9abbe"),
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewAccessApplication(ctx, "exampleAccessApplication", &cloudflare.AccessApplicationArgs{
//! 			ZoneId:          pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Name:            pulumi.String("name"),
//! 			Domain:          pulumi.String("name.example.com"),
//! 			Type:            pulumi.String("self_hosted"),
//! 			SessionDuration: pulumi.String("24h"),
//! 			AllowedIdps: pulumi.StringArray{
//! 				pulumi.String(exampleAccessIdentityProvider.Id),
//! 			},
//! 			AutoRedirectToIdentity: pulumi.Bool(true),
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
//! import com.pulumi.cloudflare.inputs.GetAccessIdentityProviderArgs;
//! import com.pulumi.cloudflare.AccessApplication;
//! import com.pulumi.cloudflare.AccessApplicationArgs;
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
//!         final var exampleAccessIdentityProvider = CloudflareFunctions.getAccessIdentityProvider(GetAccessIdentityProviderArgs.builder()
//!             .name("Google SSO")
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .build());
//! 
//!         var exampleAccessApplication = new AccessApplication("exampleAccessApplication", AccessApplicationArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .name("name")
//!             .domain("name.example.com")
//!             .type("self_hosted")
//!             .sessionDuration("24h")
//!             .allowedIdps(exampleAccessIdentityProvider.applyValue(getAccessIdentityProviderResult -> getAccessIdentityProviderResult.id()))
//!             .autoRedirectToIdentity(true)
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   exampleAccessApplication:
//!     type: cloudflare:AccessApplication
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       name: name
//!       domain: name.example.com
//!       type: self_hosted
//!       sessionDuration: 24h
//!       allowedIdps:
//!         - ${exampleAccessIdentityProvider.id}
//!       autoRedirectToIdentity: true
//! variables:
//!   exampleAccessIdentityProvider:
//!     fn::invoke:
//!       Function: cloudflare:getAccessIdentityProvider
//!       Arguments:
//!         name: Google SSO
//!         accountId: f037e56e89293a057740de681ac9abbe
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetAccessIdentityProviderArgs {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Access Identity Provider name to search for.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetAccessIdentityProviderResult {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Access Identity Provider name to search for.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Access Identity Provider Type.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetAccessIdentityProviderArgs) -> GetAccessIdentityProviderResult {

    let result = crate::bindings::pulumi::cloudflare::get_access_identity_provider::invoke(&crate::bindings::pulumi::cloudflare::get_access_identity_provider::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    GetAccessIdentityProviderResult {
        account_id: crate::into_domain(result.account_id),
        id: crate::into_domain(result.id),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}

//! Provides a Cloudflare Access Identity Provider resource. Identity
//! Providers are used as an authentication or authorisation source
//! within Access.
//!
//! > It's required that an `account_id` or `zone_id` is provided and in
//!    most cases using either is fine. However, if you're using a scoped
//!    access token, you must provide the argument that matches the token's
//!    scope. For example, an access token that is scoped to the "example.com"
//!    zone needs to use the `zone_id` argument.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! // one time pin
//! const pinLogin = new cloudflare.AccessIdentityProvider("pinLogin", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "PIN login",
//!     type: "onetimepin",
//! });
//! // oauth
//! const githubOauth = new cloudflare.AccessIdentityProvider("githubOauth", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     configs: [{
//!         clientId: "example",
//!         clientSecret: "secret_key",
//!     }],
//!     name: "GitHub OAuth",
//!     type: "github",
//! });
//! // saml
//! const jumpcloudSaml = new cloudflare.AccessIdentityProvider("jumpcloudSaml", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     configs: [{
//!         attributes: [
//!             "email",
//!             "username",
//!         ],
//!         idpPublicCert: `MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg
//! uTnQyuOqPuHbnN83d/2l1NSYKCbHt24o
//! `,
//!         issuerUrl: "jumpcloud",
//!         signRequest: false,
//!         ssoTargetUrl: "https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess",
//!     }],
//!     name: "JumpCloud SAML",
//!     type: "saml",
//! });
//! // okta
//! const okta = new cloudflare.AccessIdentityProvider("okta", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     configs: [{
//!         apiToken: "okta_api_token",
//!         clientId: "example",
//!         clientSecret: "secret_key",
//!         oktaAccount: "https://example.com",
//!     }],
//!     name: "Okta",
//!     type: "okta",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! # one time pin
//! pin_login = cloudflare.AccessIdentityProvider("pinLogin",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="PIN login",
//!     type="onetimepin")
//! # oauth
//! github_oauth = cloudflare.AccessIdentityProvider("githubOauth",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     configs=[cloudflare.AccessIdentityProviderConfigArgs(
//!         client_id="example",
//!         client_secret="secret_key",
//!     )],
//!     name="GitHub OAuth",
//!     type="github")
//! # saml
//! jumpcloud_saml = cloudflare.AccessIdentityProvider("jumpcloudSaml",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     configs=[cloudflare.AccessIdentityProviderConfigArgs(
//!         attributes=[
//!             "email",
//!             "username",
//!         ],
//!         idp_public_cert="""MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg
//! uTnQyuOqPuHbnN83d/2l1NSYKCbHt24o
//! """,
//!         issuer_url="jumpcloud",
//!         sign_request=False,
//!         sso_target_url="https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess",
//!     )],
//!     name="JumpCloud SAML",
//!     type="saml")
//! # okta
//! okta = cloudflare.AccessIdentityProvider("okta",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     configs=[cloudflare.AccessIdentityProviderConfigArgs(
//!         api_token="okta_api_token",
//!         client_id="example",
//!         client_secret="secret_key",
//!         okta_account="https://example.com",
//!     )],
//!     name="Okta",
//!     type="okta")
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
//!     // one time pin
//!     var pinLogin = new Cloudflare.AccessIdentityProvider("pinLogin", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "PIN login",
//!         Type = "onetimepin",
//!     });
//!
//!     // oauth
//!     var githubOauth = new Cloudflare.AccessIdentityProvider("githubOauth", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Configs = new[]
//!         {
//!             new Cloudflare.Inputs.AccessIdentityProviderConfigArgs
//!             {
//!                 ClientId = "example",
//!                 ClientSecret = "secret_key",
//!             },
//!         },
//!         Name = "GitHub OAuth",
//!         Type = "github",
//!     });
//!
//!     // saml
//!     var jumpcloudSaml = new Cloudflare.AccessIdentityProvider("jumpcloudSaml", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Configs = new[]
//!         {
//!             new Cloudflare.Inputs.AccessIdentityProviderConfigArgs
//!             {
//!                 Attributes = new[]
//!                 {
//!                     "email",
//!                     "username",
//!                 },
//!                 IdpPublicCert = @"MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg
//! uTnQyuOqPuHbnN83d/2l1NSYKCbHt24o
//! ",
//!                 IssuerUrl = "jumpcloud",
//!                 SignRequest = false,
//!                 SsoTargetUrl = "https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess",
//!             },
//!         },
//!         Name = "JumpCloud SAML",
//!         Type = "saml",
//!     });
//!
//!     // okta
//!     var okta = new Cloudflare.AccessIdentityProvider("okta", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Configs = new[]
//!         {
//!             new Cloudflare.Inputs.AccessIdentityProviderConfigArgs
//!             {
//!                 ApiToken = "okta_api_token",
//!                 ClientId = "example",
//!                 ClientSecret = "secret_key",
//!                 OktaAccount = "https://example.com",
//!             },
//!         },
//!         Name = "Okta",
//!         Type = "okta",
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
//! 		// one time pin
//! 		_, err := cloudflare.NewAccessIdentityProvider(ctx, "pinLogin", &cloudflare.AccessIdentityProviderArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("PIN login"),
//! 			Type:      pulumi.String("onetimepin"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// oauth
//! 		_, err = cloudflare.NewAccessIdentityProvider(ctx, "githubOauth", &cloudflare.AccessIdentityProviderArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Configs: cloudflare.AccessIdentityProviderConfigArray{
//! 				&cloudflare.AccessIdentityProviderConfigArgs{
//! 					ClientId:     pulumi.String("example"),
//! 					ClientSecret: pulumi.String("secret_key"),
//! 				},
//! 			},
//! 			Name: pulumi.String("GitHub OAuth"),
//! 			Type: pulumi.String("github"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// saml
//! 		_, err = cloudflare.NewAccessIdentityProvider(ctx, "jumpcloudSaml", &cloudflare.AccessIdentityProviderArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Configs: cloudflare.AccessIdentityProviderConfigArray{
//! 				&cloudflare.AccessIdentityProviderConfigArgs{
//! 					Attributes: pulumi.StringArray{
//! 						pulumi.String("email"),
//! 						pulumi.String("username"),
//! 					},
//! 					IdpPublicCert: pulumi.String("MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg\nuTnQyuOqPuHbnN83d/2l1NSYKCbHt24o\n"),
//! 					IssuerUrl:     pulumi.String("jumpcloud"),
//! 					SignRequest:   pulumi.Bool(false),
//! 					SsoTargetUrl:  pulumi.String("https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess"),
//! 				},
//! 			},
//! 			Name: pulumi.String("JumpCloud SAML"),
//! 			Type: pulumi.String("saml"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// okta
//! 		_, err = cloudflare.NewAccessIdentityProvider(ctx, "okta", &cloudflare.AccessIdentityProviderArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Configs: cloudflare.AccessIdentityProviderConfigArray{
//! 				&cloudflare.AccessIdentityProviderConfigArgs{
//! 					ApiToken:     pulumi.String("okta_api_token"),
//! 					ClientId:     pulumi.String("example"),
//! 					ClientSecret: pulumi.String("secret_key"),
//! 					OktaAccount:  pulumi.String("https://example.com"),
//! 				},
//! 			},
//! 			Name: pulumi.String("Okta"),
//! 			Type: pulumi.String("okta"),
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
//! import com.pulumi.cloudflare.AccessIdentityProvider;
//! import com.pulumi.cloudflare.AccessIdentityProviderArgs;
//! import com.pulumi.cloudflare.inputs.AccessIdentityProviderConfigArgs;
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
//!         // one time pin
//!         var pinLogin = new AccessIdentityProvider("pinLogin", AccessIdentityProviderArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("PIN login")
//!             .type("onetimepin")
//!             .build());
//!
//!         // oauth
//!         var githubOauth = new AccessIdentityProvider("githubOauth", AccessIdentityProviderArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .configs(AccessIdentityProviderConfigArgs.builder()
//!                 .clientId("example")
//!                 .clientSecret("secret_key")
//!                 .build())
//!             .name("GitHub OAuth")
//!             .type("github")
//!             .build());
//!
//!         // saml
//!         var jumpcloudSaml = new AccessIdentityProvider("jumpcloudSaml", AccessIdentityProviderArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .configs(AccessIdentityProviderConfigArgs.builder()
//!                 .attributes(                
//!                     "email",
//!                     "username")
//!                 .idpPublicCert("""
//! MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg
//! uTnQyuOqPuHbnN83d/2l1NSYKCbHt24o
//!                 """)
//!                 .issuerUrl("jumpcloud")
//!                 .signRequest(false)
//!                 .ssoTargetUrl("https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess")
//!                 .build())
//!             .name("JumpCloud SAML")
//!             .type("saml")
//!             .build());
//!
//!         // okta
//!         var okta = new AccessIdentityProvider("okta", AccessIdentityProviderArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .configs(AccessIdentityProviderConfigArgs.builder()
//!                 .apiToken("okta_api_token")
//!                 .clientId("example")
//!                 .clientSecret("secret_key")
//!                 .oktaAccount("https://example.com")
//!                 .build())
//!             .name("Okta")
//!             .type("okta")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # one time pin
//!   pinLogin:
//!     type: cloudflare:AccessIdentityProvider
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: PIN login
//!       type: onetimepin
//!   # oauth
//!   githubOauth:
//!     type: cloudflare:AccessIdentityProvider
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       configs:
//!         - clientId: example
//!           clientSecret: secret_key
//!       name: GitHub OAuth
//!       type: github
//!   # saml
//!   jumpcloudSaml:
//!     type: cloudflare:AccessIdentityProvider
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       configs:
//!         - attributes:
//!             - email
//!             - username
//!           idpPublicCert: |
//!             MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg
//!             uTnQyuOqPuHbnN83d/2l1NSYKCbHt24o
//!           issuerUrl: jumpcloud
//!           signRequest: false
//!           ssoTargetUrl: https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess
//!       name: JumpCloud SAML
//!       type: saml
//!   # okta
//!   okta:
//!     type: cloudflare:AccessIdentityProvider
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       configs:
//!         - apiToken: okta_api_token
//!           clientId: example
//!           clientSecret: secret_key
//!           oktaAccount: https://example.com
//!       name: Okta
//!       type: okta
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/accessIdentityProvider:AccessIdentityProvider example <account_id>/<identity_provider_id>
//! ```
//!

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AccessIdentityProviderArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Provider configuration from the [developer documentation](https://developers.cloudflare.com/access/configuring-identity-providers/).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessIdentityProviderConfig>>>,
    /// Friendly name of the Access Identity Provider configuration.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Configuration for SCIM settings for a given IDP.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub scim_configs:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessIdentityProviderScimConfig>>>,
    /// The provider type to use. Available values: `azureAD`, `centrify`, `facebook`, `github`, `google`, `google-apps`, `linkedin`, `oidc`, `okta`, `onelogin`, `onetimepin`, `pingone`, `saml`, `yandex`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessIdentityProviderResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Provider configuration from the [developer documentation](https://developers.cloudflare.com/access/configuring-identity-providers/).
    pub configs: pulumi_wasm_rust::Output<Vec<crate::types::AccessIdentityProviderConfig>>,
    /// Friendly name of the Access Identity Provider configuration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Configuration for SCIM settings for a given IDP.
    pub scim_configs: pulumi_wasm_rust::Output<Vec<crate::types::AccessIdentityProviderScimConfig>>,
    /// The provider type to use. Available values: `azureAD`, `centrify`, `facebook`, `github`, `google`, `google-apps`, `linkedin`, `oidc`, `okta`, `onelogin`, `onetimepin`, `pingone`, `saml`, `yandex`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessIdentityProviderArgs) -> AccessIdentityProviderResult {
    let result = crate::bindings::pulumi::cloudflare::access_identity_provider::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_identity_provider::Args {
            account_id: &args.account_id.get_inner(),
            configs: &args.configs.get_inner(),
            name: &args.name.get_inner(),
            scim_configs: &args.scim_configs.get_inner(),
            type_: &args.type_.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    AccessIdentityProviderResult {
        account_id: crate::into_domain(result.account_id),
        configs: crate::into_domain(result.configs),
        name: crate::into_domain(result.name),
        scim_configs: crate::into_domain(result.scim_configs),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}

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
//! const pinLogin = new cloudflare.AccessIdentityProvider("pin_login", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "PIN login",
//!     type: "onetimepin",
//! });
//! // oauth
//! const githubOauth = new cloudflare.AccessIdentityProvider("github_oauth", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "GitHub OAuth",
//!     type: "github",
//!     configs: [{
//!         clientId: "example",
//!         clientSecret: "secret_key",
//!     }],
//! });
//! // saml
//! const jumpcloudSaml = new cloudflare.AccessIdentityProvider("jumpcloud_saml", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "JumpCloud SAML",
//!     type: "saml",
//!     configs: [{
//!         issuerUrl: "jumpcloud",
//!         ssoTargetUrl: "https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess",
//!         attributes: [
//!             "email",
//!             "username",
//!         ],
//!         signRequest: false,
//!         idpPublicCert: `MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg
//! uTnQyuOqPuHbnN83d/2l1NSYKCbHt24o`,
//!     }],
//! });
//! // okta
//! const okta = new cloudflare.AccessIdentityProvider("okta", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "Okta",
//!     type: "okta",
//!     configs: [{
//!         clientId: "example",
//!         clientSecret: "secret_key",
//!         apiToken: "okta_api_token",
//!         oktaAccount: "https://example.com",
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # one time pin
//! pin_login = cloudflare.AccessIdentityProvider("pin_login",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="PIN login",
//!     type="onetimepin")
//! # oauth
//! github_oauth = cloudflare.AccessIdentityProvider("github_oauth",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="GitHub OAuth",
//!     type="github",
//!     configs=[{
//!         "client_id": "example",
//!         "client_secret": "secret_key",
//!     }])
//! # saml
//! jumpcloud_saml = cloudflare.AccessIdentityProvider("jumpcloud_saml",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="JumpCloud SAML",
//!     type="saml",
//!     configs=[{
//!         "issuer_url": "jumpcloud",
//!         "sso_target_url": "https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess",
//!         "attributes": [
//!             "email",
//!             "username",
//!         ],
//!         "sign_request": False,
//!         "idp_public_cert": """MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg
//! uTnQyuOqPuHbnN83d/2l1NSYKCbHt24o""",
//!     }])
//! # okta
//! okta = cloudflare.AccessIdentityProvider("okta",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="Okta",
//!     type="okta",
//!     configs=[{
//!         "client_id": "example",
//!         "client_secret": "secret_key",
//!         "api_token": "okta_api_token",
//!         "okta_account": "https://example.com",
//!     }])
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
//!     var pinLogin = new Cloudflare.AccessIdentityProvider("pin_login", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "PIN login",
//!         Type = "onetimepin",
//!     });
//! 
//!     // oauth
//!     var githubOauth = new Cloudflare.AccessIdentityProvider("github_oauth", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "GitHub OAuth",
//!         Type = "github",
//!         Configs = new[]
//!         {
//!             new Cloudflare.Inputs.AccessIdentityProviderConfigArgs
//!             {
//!                 ClientId = "example",
//!                 ClientSecret = "secret_key",
//!             },
//!         },
//!     });
//! 
//!     // saml
//!     var jumpcloudSaml = new Cloudflare.AccessIdentityProvider("jumpcloud_saml", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "JumpCloud SAML",
//!         Type = "saml",
//!         Configs = new[]
//!         {
//!             new Cloudflare.Inputs.AccessIdentityProviderConfigArgs
//!             {
//!                 IssuerUrl = "jumpcloud",
//!                 SsoTargetUrl = "https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess",
//!                 Attributes = new[]
//!                 {
//!                     "email",
//!                     "username",
//!                 },
//!                 SignRequest = false,
//!                 IdpPublicCert = @"MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg
//! uTnQyuOqPuHbnN83d/2l1NSYKCbHt24o",
//!             },
//!         },
//!     });
//! 
//!     // okta
//!     var okta = new Cloudflare.AccessIdentityProvider("okta", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "Okta",
//!         Type = "okta",
//!         Configs = new[]
//!         {
//!             new Cloudflare.Inputs.AccessIdentityProviderConfigArgs
//!             {
//!                 ClientId = "example",
//!                 ClientSecret = "secret_key",
//!                 ApiToken = "okta_api_token",
//!                 OktaAccount = "https://example.com",
//!             },
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
//! 		// one time pin
//! 		_, err := cloudflare.NewAccessIdentityProvider(ctx, "pin_login", &cloudflare.AccessIdentityProviderArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("PIN login"),
//! 			Type:      pulumi.String("onetimepin"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// oauth
//! 		_, err = cloudflare.NewAccessIdentityProvider(ctx, "github_oauth", &cloudflare.AccessIdentityProviderArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("GitHub OAuth"),
//! 			Type:      pulumi.String("github"),
//! 			Configs: cloudflare.AccessIdentityProviderConfigArray{
//! 				&cloudflare.AccessIdentityProviderConfigArgs{
//! 					ClientId:     pulumi.String("example"),
//! 					ClientSecret: pulumi.String("secret_key"),
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// saml
//! 		_, err = cloudflare.NewAccessIdentityProvider(ctx, "jumpcloud_saml", &cloudflare.AccessIdentityProviderArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("JumpCloud SAML"),
//! 			Type:      pulumi.String("saml"),
//! 			Configs: cloudflare.AccessIdentityProviderConfigArray{
//! 				&cloudflare.AccessIdentityProviderConfigArgs{
//! 					IssuerUrl:    pulumi.String("jumpcloud"),
//! 					SsoTargetUrl: pulumi.String("https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess"),
//! 					Attributes: pulumi.StringArray{
//! 						pulumi.String("email"),
//! 						pulumi.String("username"),
//! 					},
//! 					SignRequest:   pulumi.Bool(false),
//! 					IdpPublicCert: pulumi.String("MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg\nuTnQyuOqPuHbnN83d/2l1NSYKCbHt24o"),
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// okta
//! 		_, err = cloudflare.NewAccessIdentityProvider(ctx, "okta", &cloudflare.AccessIdentityProviderArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("Okta"),
//! 			Type:      pulumi.String("okta"),
//! 			Configs: cloudflare.AccessIdentityProviderConfigArray{
//! 				&cloudflare.AccessIdentityProviderConfigArgs{
//! 					ClientId:     pulumi.String("example"),
//! 					ClientSecret: pulumi.String("secret_key"),
//! 					ApiToken:     pulumi.String("okta_api_token"),
//! 					OktaAccount:  pulumi.String("https://example.com"),
//! 				},
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
//!             .name("GitHub OAuth")
//!             .type("github")
//!             .configs(AccessIdentityProviderConfigArgs.builder()
//!                 .clientId("example")
//!                 .clientSecret("secret_key")
//!                 .build())
//!             .build());
//! 
//!         // saml
//!         var jumpcloudSaml = new AccessIdentityProvider("jumpcloudSaml", AccessIdentityProviderArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("JumpCloud SAML")
//!             .type("saml")
//!             .configs(AccessIdentityProviderConfigArgs.builder()
//!                 .issuerUrl("jumpcloud")
//!                 .ssoTargetUrl("https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess")
//!                 .attributes(                
//!                     "email",
//!                     "username")
//!                 .signRequest(false)
//!                 .idpPublicCert("""
//! MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg
//! uTnQyuOqPuHbnN83d/2l1NSYKCbHt24o                """)
//!                 .build())
//!             .build());
//! 
//!         // okta
//!         var okta = new AccessIdentityProvider("okta", AccessIdentityProviderArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("Okta")
//!             .type("okta")
//!             .configs(AccessIdentityProviderConfigArgs.builder()
//!                 .clientId("example")
//!                 .clientSecret("secret_key")
//!                 .apiToken("okta_api_token")
//!                 .oktaAccount("https://example.com")
//!                 .build())
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
//!     name: pin_login
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: PIN login
//!       type: onetimepin
//!   # oauth
//!   githubOauth:
//!     type: cloudflare:AccessIdentityProvider
//!     name: github_oauth
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: GitHub OAuth
//!       type: github
//!       configs:
//!         - clientId: example
//!           clientSecret: secret_key
//!   # saml
//!   jumpcloudSaml:
//!     type: cloudflare:AccessIdentityProvider
//!     name: jumpcloud_saml
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: JumpCloud SAML
//!       type: saml
//!       configs:
//!         - issuerUrl: jumpcloud
//!           ssoTargetUrl: https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess
//!           attributes:
//!             - email
//!             - username
//!           signRequest: false
//!           idpPublicCert: |-
//!             MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg
//!             uTnQyuOqPuHbnN83d/2l1NSYKCbHt24o
//!   # okta
//!   okta:
//!     type: cloudflare:AccessIdentityProvider
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: Okta
//!       type: okta
//!       configs:
//!         - clientId: example
//!           clientSecret: secret_key
//!           apiToken: okta_api_token
//!           oktaAccount: https://example.com
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
    pub scim_configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessIdentityProviderScimConfig>>>,
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

    let result = crate::bindings::pulumi::cloudflare::access_identity_provider::invoke(name, &crate::bindings::pulumi::cloudflare::access_identity_provider::Args {
        account_id: &args.account_id.get_inner(),
        configs: &args.configs.get_inner(),
        name: &args.name.get_inner(),
        scim_configs: &args.scim_configs.get_inner(),
        type_: &args.type_.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    AccessIdentityProviderResult {
        account_id: crate::into_domain(result.account_id),
        configs: crate::into_domain(result.configs),
        name: crate::into_domain(result.name),
        scim_configs: crate::into_domain(result.scim_configs),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}

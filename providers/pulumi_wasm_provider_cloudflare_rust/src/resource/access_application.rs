//! Provides a Cloudflare Access Application resource. Access
//! Applications are used to restrict access to a whole application using an
//! authorisation gateway managed by Cloudflare.
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
//! // With CORS configuration
//! const stagingApp = new cloudflare.AccessApplication("stagingApp", {
//!     corsHeaders: [{
//!         allowCredentials: true,
//!         allowedMethods: [
//!             "GET",
//!             "POST",
//!             "OPTIONS",
//!         ],
//!         allowedOrigins: ["https://example.com"],
//!         maxAge: 10,
//!     }],
//!     domain: "staging.example.com",
//!     name: "staging application",
//!     sessionDuration: "24h",
//!     type: "self_hosted",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # With CORS configuration
//! staging_app = cloudflare.AccessApplication("stagingApp",
//!     cors_headers=[cloudflare.AccessApplicationCorsHeaderArgs(
//!         allow_credentials=True,
//!         allowed_methods=[
//!             "GET",
//!             "POST",
//!             "OPTIONS",
//!         ],
//!         allowed_origins=["https://example.com"],
//!         max_age=10,
//!     )],
//!     domain="staging.example.com",
//!     name="staging application",
//!     session_duration="24h",
//!     type="self_hosted",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711")
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
//!     // With CORS configuration
//!     var stagingApp = new Cloudflare.AccessApplication("stagingApp", new()
//!     {
//!         CorsHeaders = new[]
//!         {
//!             new Cloudflare.Inputs.AccessApplicationCorsHeaderArgs
//!             {
//!                 AllowCredentials = true,
//!                 AllowedMethods = new[]
//!                 {
//!                     "GET",
//!                     "POST",
//!                     "OPTIONS",
//!                 },
//!                 AllowedOrigins = new[]
//!                 {
//!                     "https://example.com",
//!                 },
//!                 MaxAge = 10,
//!             },
//!         },
//!         Domain = "staging.example.com",
//!         Name = "staging application",
//!         SessionDuration = "24h",
//!         Type = "self_hosted",
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
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
//! 		// With CORS configuration
//! 		_, err := cloudflare.NewAccessApplication(ctx, "stagingApp", &cloudflare.AccessApplicationArgs{
//! 			CorsHeaders: cloudflare.AccessApplicationCorsHeaderArray{
//! 				&cloudflare.AccessApplicationCorsHeaderArgs{
//! 					AllowCredentials: pulumi.Bool(true),
//! 					AllowedMethods: pulumi.StringArray{
//! 						pulumi.String("GET"),
//! 						pulumi.String("POST"),
//! 						pulumi.String("OPTIONS"),
//! 					},
//! 					AllowedOrigins: pulumi.StringArray{
//! 						pulumi.String("https://example.com"),
//! 					},
//! 					MaxAge: pulumi.Int(10),
//! 				},
//! 			},
//! 			Domain:          pulumi.String("staging.example.com"),
//! 			Name:            pulumi.String("staging application"),
//! 			SessionDuration: pulumi.String("24h"),
//! 			Type:            pulumi.String("self_hosted"),
//! 			ZoneId:          pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.AccessApplication;
//! import com.pulumi.cloudflare.AccessApplicationArgs;
//! import com.pulumi.cloudflare.inputs.AccessApplicationCorsHeaderArgs;
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
//!         // With CORS configuration
//!         var stagingApp = new AccessApplication("stagingApp", AccessApplicationArgs.builder()        
//!             .corsHeaders(AccessApplicationCorsHeaderArgs.builder()
//!                 .allowCredentials(true)
//!                 .allowedMethods(                
//!                     "GET",
//!                     "POST",
//!                     "OPTIONS")
//!                 .allowedOrigins("https://example.com")
//!                 .maxAge(10)
//!                 .build())
//!             .domain("staging.example.com")
//!             .name("staging application")
//!             .sessionDuration("24h")
//!             .type("self_hosted")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # With CORS configuration
//!   stagingApp:
//!     type: cloudflare:AccessApplication
//!     properties:
//!       corsHeaders:
//!         - allowCredentials: true
//!           allowedMethods:
//!             - GET
//!             - POST
//!             - OPTIONS
//!           allowedOrigins:
//!             - https://example.com
//!           maxAge: 10
//!       domain: staging.example.com
//!       name: staging application
//!       sessionDuration: 24h
//!       type: self_hosted
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessApplication:AccessApplication example <account_id>/<application_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AccessApplicationArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// When set to true, users can authenticate to this application using their WARP session. When set to false this application will always require direct IdP authentication. This setting always overrides the organization setting for WARP authentication.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
    /// The identity providers selected for the application.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub allowed_idps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The logo URL of the app launcher.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub app_launcher_logo_url: pulumi_wasm_rust::Output<Option<String>>,
    /// Option to show/hide applications in App Launcher. Defaults to `true`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub app_launcher_visible: pulumi_wasm_rust::Output<Option<bool>>,
    /// Option to skip identity provider selection if only one is configured in `allowed_idps`. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
    /// The background color of the app launcher.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub bg_color: pulumi_wasm_rust::Output<Option<String>>,
    /// CORS configuration for the Access Application. See below for reference structure.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub cors_headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessApplicationCorsHeader>>>,
    /// Option that returns a custom error message when a user is denied access to the application.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_deny_message: pulumi_wasm_rust::Output<Option<String>>,
    /// Option that redirects to a custom URL when a user is denied access to the application via identity based rules.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_deny_url: pulumi_wasm_rust::Output<Option<String>>,
    /// Option that redirects to a custom URL when a user is denied access to the application via non identity rules.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_non_identity_deny_url: pulumi_wasm_rust::Output<Option<String>>,
    /// The custom pages selected for the application.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_pages: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The primary hostname and path that Access will secure. If the app is visible in the App Launcher dashboard, this is the domain that will be displayed.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub domain: pulumi_wasm_rust::Output<Option<String>>,
    /// Option to provide increased security against compromised authorization tokens and CSRF attacks by requiring an additional "binding" cookie on requests. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enable_binding_cookie: pulumi_wasm_rust::Output<Option<bool>>,
    /// The footer links of the app launcher.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub footer_links: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessApplicationFooterLink>>>,
    /// The background color of the header bar in the app launcher.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub header_bg_color: pulumi_wasm_rust::Output<Option<String>>,
    /// Option to add the `HttpOnly` cookie flag to access tokens.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub http_only_cookie_attribute: pulumi_wasm_rust::Output<Option<bool>>,
    /// The landing page design of the app launcher.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub landing_page_design: pulumi_wasm_rust::Output<Option<crate::types::AccessApplicationLandingPageDesign>>,
    /// Image URL for the logo shown in the app launcher dashboard.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub logo_url: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the footer link.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// SaaS configuration for the Access Application.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub saas_app: pulumi_wasm_rust::Output<Option<crate::types::AccessApplicationSaasApp>>,
    /// Defines the same-site cookie setting for access tokens. Available values: `none`, `lax`, `strict`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub same_site_cookie_attribute: pulumi_wasm_rust::Output<Option<String>>,
    /// List of domains that access will secure. Only present for self_hosted, vnc, and ssh applications. Always includes the value set as `domain`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub self_hosted_domains: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Option to return a 401 status code in service authentication rules on failed requests. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub service_auth401_redirect: pulumi_wasm_rust::Output<Option<bool>>,
    /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`. Defaults to `24h`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// Option to skip the authorization interstitial when using the CLI. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub skip_interstitial: pulumi_wasm_rust::Output<Option<bool>>,
    /// The itags associated with the application.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The application type. Available values: `app_launcher`, `bookmark`, `biso`, `dash_sso`, `saas`, `self_hosted`, `ssh`, `vnc`, `warp`. Defaults to `self_hosted`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessApplicationResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// When set to true, users can authenticate to this application using their WARP session. When set to false this application will always require direct IdP authentication. This setting always overrides the organization setting for WARP authentication.
    pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
    /// The identity providers selected for the application.
    pub allowed_idps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The logo URL of the app launcher.
    pub app_launcher_logo_url: pulumi_wasm_rust::Output<Option<String>>,
    /// Option to show/hide applications in App Launcher. Defaults to `true`.
    pub app_launcher_visible: pulumi_wasm_rust::Output<Option<bool>>,
    /// Application Audience (AUD) Tag of the application.
    pub aud: pulumi_wasm_rust::Output<String>,
    /// Option to skip identity provider selection if only one is configured in `allowed_idps`. Defaults to `false`.
    pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
    /// The background color of the app launcher.
    pub bg_color: pulumi_wasm_rust::Output<Option<String>>,
    /// CORS configuration for the Access Application. See below for reference structure.
    pub cors_headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessApplicationCorsHeader>>>,
    /// Option that returns a custom error message when a user is denied access to the application.
    pub custom_deny_message: pulumi_wasm_rust::Output<Option<String>>,
    /// Option that redirects to a custom URL when a user is denied access to the application via identity based rules.
    pub custom_deny_url: pulumi_wasm_rust::Output<Option<String>>,
    /// Option that redirects to a custom URL when a user is denied access to the application via non identity rules.
    pub custom_non_identity_deny_url: pulumi_wasm_rust::Output<Option<String>>,
    /// The custom pages selected for the application.
    pub custom_pages: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The primary hostname and path that Access will secure. If the app is visible in the App Launcher dashboard, this is the domain that will be displayed.
    pub domain: pulumi_wasm_rust::Output<String>,
    /// Option to provide increased security against compromised authorization tokens and CSRF attacks by requiring an additional "binding" cookie on requests. Defaults to `false`.
    pub enable_binding_cookie: pulumi_wasm_rust::Output<Option<bool>>,
    /// The footer links of the app launcher.
    pub footer_links: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessApplicationFooterLink>>>,
    /// The background color of the header bar in the app launcher.
    pub header_bg_color: pulumi_wasm_rust::Output<Option<String>>,
    /// Option to add the `HttpOnly` cookie flag to access tokens.
    pub http_only_cookie_attribute: pulumi_wasm_rust::Output<Option<bool>>,
    /// The landing page design of the app launcher.
    pub landing_page_design: pulumi_wasm_rust::Output<Option<crate::types::AccessApplicationLandingPageDesign>>,
    /// Image URL for the logo shown in the app launcher dashboard.
    pub logo_url: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the footer link.
    pub name: pulumi_wasm_rust::Output<String>,
    /// SaaS configuration for the Access Application.
    pub saas_app: pulumi_wasm_rust::Output<Option<crate::types::AccessApplicationSaasApp>>,
    /// Defines the same-site cookie setting for access tokens. Available values: `none`, `lax`, `strict`.
    pub same_site_cookie_attribute: pulumi_wasm_rust::Output<Option<String>>,
    /// List of domains that access will secure. Only present for self_hosted, vnc, and ssh applications. Always includes the value set as `domain`.
    pub self_hosted_domains: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Option to return a 401 status code in service authentication rules on failed requests. Defaults to `false`.
    pub service_auth401_redirect: pulumi_wasm_rust::Output<Option<bool>>,
    /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`. Defaults to `24h`.
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// Option to skip the authorization interstitial when using the CLI. Defaults to `false`.
    pub skip_interstitial: pulumi_wasm_rust::Output<Option<bool>>,
    /// The itags associated with the application.
    pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The application type. Available values: `app_launcher`, `bookmark`, `biso`, `dash_sso`, `saas`, `self_hosted`, `ssh`, `vnc`, `warp`. Defaults to `self_hosted`.
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessApplicationArgs) -> AccessApplicationResult {

    let result = crate::bindings::pulumi::cloudflare::access_application::invoke(name, &crate::bindings::pulumi::cloudflare::access_application::Args {
        account_id: &args.account_id.get_inner(),
        allow_authenticate_via_warp: &args.allow_authenticate_via_warp.get_inner(),
        allowed_idps: &args.allowed_idps.get_inner(),
        app_launcher_logo_url: &args.app_launcher_logo_url.get_inner(),
        app_launcher_visible: &args.app_launcher_visible.get_inner(),
        auto_redirect_to_identity: &args.auto_redirect_to_identity.get_inner(),
        bg_color: &args.bg_color.get_inner(),
        cors_headers: &args.cors_headers.get_inner(),
        custom_deny_message: &args.custom_deny_message.get_inner(),
        custom_deny_url: &args.custom_deny_url.get_inner(),
        custom_non_identity_deny_url: &args.custom_non_identity_deny_url.get_inner(),
        custom_pages: &args.custom_pages.get_inner(),
        domain: &args.domain.get_inner(),
        enable_binding_cookie: &args.enable_binding_cookie.get_inner(),
        footer_links: &args.footer_links.get_inner(),
        header_bg_color: &args.header_bg_color.get_inner(),
        http_only_cookie_attribute: &args.http_only_cookie_attribute.get_inner(),
        landing_page_design: &args.landing_page_design.get_inner(),
        logo_url: &args.logo_url.get_inner(),
        name: &args.name.get_inner(),
        saas_app: &args.saas_app.get_inner(),
        same_site_cookie_attribute: &args.same_site_cookie_attribute.get_inner(),
        self_hosted_domains: &args.self_hosted_domains.get_inner(),
        service_auth401_redirect: &args.service_auth401_redirect.get_inner(),
        session_duration: &args.session_duration.get_inner(),
        skip_interstitial: &args.skip_interstitial.get_inner(),
        tags: &args.tags.get_inner(),
        type_: &args.type_.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    AccessApplicationResult {
        account_id: crate::into_domain(result.account_id),
        allow_authenticate_via_warp: crate::into_domain(result.allow_authenticate_via_warp),
        allowed_idps: crate::into_domain(result.allowed_idps),
        app_launcher_logo_url: crate::into_domain(result.app_launcher_logo_url),
        app_launcher_visible: crate::into_domain(result.app_launcher_visible),
        aud: crate::into_domain(result.aud),
        auto_redirect_to_identity: crate::into_domain(result.auto_redirect_to_identity),
        bg_color: crate::into_domain(result.bg_color),
        cors_headers: crate::into_domain(result.cors_headers),
        custom_deny_message: crate::into_domain(result.custom_deny_message),
        custom_deny_url: crate::into_domain(result.custom_deny_url),
        custom_non_identity_deny_url: crate::into_domain(result.custom_non_identity_deny_url),
        custom_pages: crate::into_domain(result.custom_pages),
        domain: crate::into_domain(result.domain),
        enable_binding_cookie: crate::into_domain(result.enable_binding_cookie),
        footer_links: crate::into_domain(result.footer_links),
        header_bg_color: crate::into_domain(result.header_bg_color),
        http_only_cookie_attribute: crate::into_domain(result.http_only_cookie_attribute),
        landing_page_design: crate::into_domain(result.landing_page_design),
        logo_url: crate::into_domain(result.logo_url),
        name: crate::into_domain(result.name),
        saas_app: crate::into_domain(result.saas_app),
        same_site_cookie_attribute: crate::into_domain(result.same_site_cookie_attribute),
        self_hosted_domains: crate::into_domain(result.self_hosted_domains),
        service_auth401_redirect: crate::into_domain(result.service_auth401_redirect),
        session_duration: crate::into_domain(result.session_duration),
        skip_interstitial: crate::into_domain(result.skip_interstitial),
        tags: crate::into_domain(result.tags),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}

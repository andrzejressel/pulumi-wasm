//! A Zero Trust organization defines the user login experience.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.AccessOrganization("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     authDomain: "example.cloudflareaccess.com",
//!     autoRedirectToIdentity: false,
//!     isUiReadOnly: false,
//!     loginDesigns: [{
//!         backgroundColor: "#ffffff",
//!         footerText: "My footer text",
//!         headerText: "My header text",
//!         logoPath: "https://example.com/logo.png",
//!         textColor: "#000000",
//!     }],
//!     name: "example.cloudflareaccess.com",
//!     userSeatExpirationInactiveTime: "720h",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.AccessOrganization("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     auth_domain="example.cloudflareaccess.com",
//!     auto_redirect_to_identity=False,
//!     is_ui_read_only=False,
//!     login_designs=[cloudflare.AccessOrganizationLoginDesignArgs(
//!         background_color="#ffffff",
//!         footer_text="My footer text",
//!         header_text="My header text",
//!         logo_path="https://example.com/logo.png",
//!         text_color="#000000",
//!     )],
//!     name="example.cloudflareaccess.com",
//!     user_seat_expiration_inactive_time="720h")
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
//!     var example = new Cloudflare.AccessOrganization("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         AuthDomain = "example.cloudflareaccess.com",
//!         AutoRedirectToIdentity = false,
//!         IsUiReadOnly = false,
//!         LoginDesigns = new[]
//!         {
//!             new Cloudflare.Inputs.AccessOrganizationLoginDesignArgs
//!             {
//!                 BackgroundColor = "#ffffff",
//!                 FooterText = "My footer text",
//!                 HeaderText = "My header text",
//!                 LogoPath = "https://example.com/logo.png",
//!                 TextColor = "#000000",
//!             },
//!         },
//!         Name = "example.cloudflareaccess.com",
//!         UserSeatExpirationInactiveTime = "720h",
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
//! 		_, err := cloudflare.NewAccessOrganization(ctx, "example", &cloudflare.AccessOrganizationArgs{
//! 			AccountId:              pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			AuthDomain:             pulumi.String("example.cloudflareaccess.com"),
//! 			AutoRedirectToIdentity: pulumi.Bool(false),
//! 			IsUiReadOnly:           pulumi.Bool(false),
//! 			LoginDesigns: cloudflare.AccessOrganizationLoginDesignArray{
//! 				&cloudflare.AccessOrganizationLoginDesignArgs{
//! 					BackgroundColor: pulumi.String("#ffffff"),
//! 					FooterText:      pulumi.String("My footer text"),
//! 					HeaderText:      pulumi.String("My header text"),
//! 					LogoPath:        pulumi.String("https://example.com/logo.png"),
//! 					TextColor:       pulumi.String("#000000"),
//! 				},
//! 			},
//! 			Name:                           pulumi.String("example.cloudflareaccess.com"),
//! 			UserSeatExpirationInactiveTime: pulumi.String("720h"),
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
//! import com.pulumi.cloudflare.AccessOrganization;
//! import com.pulumi.cloudflare.AccessOrganizationArgs;
//! import com.pulumi.cloudflare.inputs.AccessOrganizationLoginDesignArgs;
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
//!         var example = new AccessOrganization("example", AccessOrganizationArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .authDomain("example.cloudflareaccess.com")
//!             .autoRedirectToIdentity(false)
//!             .isUiReadOnly(false)
//!             .loginDesigns(AccessOrganizationLoginDesignArgs.builder()
//!                 .backgroundColor("#ffffff")
//!                 .footerText("My footer text")
//!                 .headerText("My header text")
//!                 .logoPath("https://example.com/logo.png")
//!                 .textColor("#000000")
//!                 .build())
//!             .name("example.cloudflareaccess.com")
//!             .userSeatExpirationInactiveTime("720h")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:AccessOrganization
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       authDomain: example.cloudflareaccess.com
//!       autoRedirectToIdentity: false
//!       isUiReadOnly: false
//!       loginDesigns:
//!         - backgroundColor: '#ffffff'
//!           footerText: My footer text
//!           headerText: My header text
//!           logoPath: https://example.com/logo.png
//!           textColor: '#000000'
//!       name: example.cloudflareaccess.com
//!       userSeatExpirationInactiveTime: 720h
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/accessOrganization:AccessOrganization example <account_id>
//! ```
//!

pub struct AccessOrganizationArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// When set to true, users can authenticate via WARP for any application in your organization. Application settings will take precedence over this value.
    pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
    /// The unique subdomain assigned to your Zero Trust organization.
    pub auth_domain: pulumi_wasm_rust::Output<String>,
    /// When set to true, users skip the identity provider selection step during login.
    pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
    /// Custom pages for your Zero Trust organization.
    pub custom_pages:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationCustomPage>>>,
    /// When set to true, this will disable all editing of Access resources via the Zero Trust Dashboard.
    pub is_ui_read_only: pulumi_wasm_rust::Output<Option<bool>>,
    pub login_designs:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationLoginDesign>>>,
    /// The name of your Zero Trust organization.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// A description of the reason why the UI read only field is being toggled.
    pub ui_read_only_toggle_reason: pulumi_wasm_rust::Output<Option<String>>,
    /// The amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`.
    pub user_seat_expiration_inactive_time: pulumi_wasm_rust::Output<Option<String>>,
    /// The amount of time that tokens issued for applications will be valid. Must be in the format 30m or 2h45m. Valid time units are: m, h.
    pub warp_auth_session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessOrganizationResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// When set to true, users can authenticate via WARP for any application in your organization. Application settings will take precedence over this value.
    pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
    /// The unique subdomain assigned to your Zero Trust organization.
    pub auth_domain: pulumi_wasm_rust::Output<String>,
    /// When set to true, users skip the identity provider selection step during login.
    pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
    /// Custom pages for your Zero Trust organization.
    pub custom_pages:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationCustomPage>>>,
    /// When set to true, this will disable all editing of Access resources via the Zero Trust Dashboard.
    pub is_ui_read_only: pulumi_wasm_rust::Output<Option<bool>>,
    pub login_designs:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationLoginDesign>>>,
    /// The name of your Zero Trust organization.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// A description of the reason why the UI read only field is being toggled.
    pub ui_read_only_toggle_reason: pulumi_wasm_rust::Output<Option<String>>,
    /// The amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`.
    pub user_seat_expiration_inactive_time: pulumi_wasm_rust::Output<Option<String>>,
    /// The amount of time that tokens issued for applications will be valid. Must be in the format 30m or 2h45m. Valid time units are: m, h.
    pub warp_auth_session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessOrganizationArgs) -> AccessOrganizationResult {
    let result = crate::bindings::pulumi::cloudflare::access_organization::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_organization::Args {
            account_id: args.account_id.get_inner(),
            allow_authenticate_via_warp: args.allow_authenticate_via_warp.get_inner(),
            auth_domain: args.auth_domain.get_inner(),
            auto_redirect_to_identity: args.auto_redirect_to_identity.get_inner(),
            custom_pages: args.custom_pages.get_inner(),
            is_ui_read_only: args.is_ui_read_only.get_inner(),
            login_designs: args.login_designs.get_inner(),
            name: args.name.get_inner(),
            session_duration: args.session_duration.get_inner(),
            ui_read_only_toggle_reason: args.ui_read_only_toggle_reason.get_inner(),
            user_seat_expiration_inactive_time: args.user_seat_expiration_inactive_time.get_inner(),
            warp_auth_session_duration: args.warp_auth_session_duration.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AccessOrganizationResult {
        account_id: crate::into_domain(result.account_id),
        allow_authenticate_via_warp: crate::into_domain(result.allow_authenticate_via_warp),
        auth_domain: crate::into_domain(result.auth_domain),
        auto_redirect_to_identity: crate::into_domain(result.auto_redirect_to_identity),
        custom_pages: crate::into_domain(result.custom_pages),
        is_ui_read_only: crate::into_domain(result.is_ui_read_only),
        login_designs: crate::into_domain(result.login_designs),
        name: crate::into_domain(result.name),
        session_duration: crate::into_domain(result.session_duration),
        ui_read_only_toggle_reason: crate::into_domain(result.ui_read_only_toggle_reason),
        user_seat_expiration_inactive_time: crate::into_domain(
            result.user_seat_expiration_inactive_time,
        ),
        warp_auth_session_duration: crate::into_domain(result.warp_auth_session_duration),
        zone_id: crate::into_domain(result.zone_id),
    }
}

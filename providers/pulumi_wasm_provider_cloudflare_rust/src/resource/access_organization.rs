//! A Zero Trust organization defines the user login experience.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = access_organization::create(
//!         "example",
//!         AccessOrganizationArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .auth_domain("example.cloudflareaccess.com")
//!             .auto_redirect_to_identity(false)
//!             .is_ui_read_only(false)
//!             .login_designs(
//!                 vec![
//!                     AccessOrganizationLoginDesign::builder().backgroundColor("#ffffff")
//!                     .footerText("My footer text").headerText("My header text")
//!                     .logoPath("https://example.com/logo.png").textColor("#000000")
//!                     .build_struct(),
//!                 ],
//!             )
//!             .name("example.cloudflareaccess.com")
//!             .user_seat_expiration_inactive_time("720h")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessOrganization:AccessOrganization example <account_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccessOrganizationArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// When set to true, users can authenticate via WARP for any application in your organization. Application settings will take precedence over this value.
    #[builder(into, default)]
    pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
    /// The unique subdomain assigned to your Zero Trust organization.
    #[builder(into)]
    pub auth_domain: pulumi_wasm_rust::Output<String>,
    /// When set to true, users skip the identity provider selection step during login.
    #[builder(into, default)]
    pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
    /// Custom pages for your Zero Trust organization.
    #[builder(into, default)]
    pub custom_pages: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationCustomPage>>>,
    /// When set to true, this will disable all editing of Access resources via the Zero Trust Dashboard.
    #[builder(into, default)]
    pub is_ui_read_only: pulumi_wasm_rust::Output<Option<bool>>,
    #[builder(into, default)]
    pub login_designs: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationLoginDesign>>>,
    /// The name of your Zero Trust organization.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
    #[builder(into, default)]
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// A description of the reason why the UI read only field is being toggled.
    #[builder(into, default)]
    pub ui_read_only_toggle_reason: pulumi_wasm_rust::Output<Option<String>>,
    /// The amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`.
    #[builder(into, default)]
    pub user_seat_expiration_inactive_time: pulumi_wasm_rust::Output<Option<String>>,
    /// The amount of time that tokens issued for applications will be valid. Must be in the format 30m or 2h45m. Valid time units are: m, h.
    #[builder(into, default)]
    pub warp_auth_session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default)]
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
    pub custom_pages: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationCustomPage>>>,
    /// When set to true, this will disable all editing of Access resources via the Zero Trust Dashboard.
    pub is_ui_read_only: pulumi_wasm_rust::Output<Option<bool>>,
    pub login_designs: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationLoginDesign>>>,
    /// The name of your Zero Trust organization.
    pub name: pulumi_wasm_rust::Output<String>,
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
pub fn create(
    name: &str,
    args: AccessOrganizationArgs
) -> AccessOrganizationResult {

    let result = crate::bindings::pulumi::cloudflare::access_organization::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_organization::Args {
                account_id: &args.account_id.get_inner(),
                allow_authenticate_via_warp: &args.allow_authenticate_via_warp.get_inner(),
                auth_domain: &args.auth_domain.get_inner(),
                auto_redirect_to_identity: &args.auto_redirect_to_identity.get_inner(),
                custom_pages: &args.custom_pages.get_inner(),
                is_ui_read_only: &args.is_ui_read_only.get_inner(),
                login_designs: &args.login_designs.get_inner(),
                name: &args.name.get_inner(),
                session_duration: &args.session_duration.get_inner(),
                ui_read_only_toggle_reason: &args.ui_read_only_toggle_reason.get_inner(),
                user_seat_expiration_inactive_time: &args.user_seat_expiration_inactive_time.get_inner(),
                warp_auth_session_duration: &args.warp_auth_session_duration.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
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
        user_seat_expiration_inactive_time: crate::into_domain(result.user_seat_expiration_inactive_time),
        warp_auth_session_duration: crate::into_domain(result.warp_auth_session_duration),
        zone_id: crate::into_domain(result.zone_id),
    }
}

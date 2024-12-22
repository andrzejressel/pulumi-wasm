//! Provides a resource to customize the pages your end users will see
//! when trying to reach applications behind Cloudflare Access.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = zero_trust_access_custom_page::create(
//!         "example",
//!         ZeroTrustAccessCustomPageArgs::builder()
//!             .custom_html("<html><body><h1>Forbidden</h1></body></html>")
//!             .name("example")
//!             .type_("forbidden")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessCustomPageArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Number of apps to display on the custom page.
    #[builder(into, default)]
    pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Custom HTML to display on the custom page.
    #[builder(into, default)]
    pub custom_html: pulumi_wasm_rust::Output<Option<String>>,
    /// Friendly name of the Access Custom Page configuration.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Type of Access custom page to create. Available values: `identity_denied`, `forbidden`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ZeroTrustAccessCustomPageResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Number of apps to display on the custom page.
    pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Custom HTML to display on the custom page.
    pub custom_html: pulumi_wasm_rust::Output<Option<String>>,
    /// Friendly name of the Access Custom Page configuration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Type of Access custom page to create. Available values: `identity_denied`, `forbidden`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ZeroTrustAccessCustomPageArgs
) -> ZeroTrustAccessCustomPageResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_access_custom_page::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zero_trust_access_custom_page::Args {
                account_id: &args.account_id.get_inner(),
                app_count: &args.app_count.get_inner(),
                custom_html: &args.custom_html.get_inner(),
                name: &args.name.get_inner(),
                type_: &args.type_.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    ZeroTrustAccessCustomPageResult {
        account_id: crate::into_domain(result.account_id),
        app_count: crate::into_domain(result.app_count),
        custom_html: crate::into_domain(result.custom_html),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}

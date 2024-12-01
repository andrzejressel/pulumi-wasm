//! Provides a resource which manages Cloudflare custom error pages.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = custom_pages::create(
//!         "example",
//!         CustomPagesArgs::builder()
//!             .state("customized")
//!             .type_("basic_challenge")
//!             .url("https://example.com/challenge.html")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/customPages:CustomPages example <resource_level>/<resource_id>/<custom_page_type>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct CustomPagesArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Managed state of the custom page. Available values: `default`, `customized`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub state: pulumi_wasm_rust::Output<Option<String>>,
    /// The type of custom page you wish to update. Available values: `basic_challenge`, `waf_challenge`, `waf_block`, `ratelimit_block`, `country_challenge`, `ip_block`, `under_attack`, `500_errors`, `1000_errors`, `managed_challenge`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// URL of where the custom page source is located.
    #[builder(into)]
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct CustomPagesResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Managed state of the custom page. Available values: `default`, `customized`.
    pub state: pulumi_wasm_rust::Output<Option<String>>,
    /// The type of custom page you wish to update. Available values: `basic_challenge`, `waf_challenge`, `waf_block`, `ratelimit_block`, `country_challenge`, `ip_block`, `under_attack`, `500_errors`, `1000_errors`, `managed_challenge`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// URL of where the custom page source is located.
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: CustomPagesArgs) -> CustomPagesResult {

    let result = crate::bindings::pulumi::cloudflare::custom_pages::invoke(name, &crate::bindings::pulumi::cloudflare::custom_pages::Args {
        account_id: &args.account_id.get_inner(),
        state: &args.state.get_inner(),
        type_: &args.type_.get_inner(),
        url: &args.url.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    CustomPagesResult {
        account_id: crate::into_domain(result.account_id),
        state: crate::into_domain(result.state),
        type_: crate::into_domain(result.type_),
        url: crate::into_domain(result.url),
        zone_id: crate::into_domain(result.zone_id),
    }
}

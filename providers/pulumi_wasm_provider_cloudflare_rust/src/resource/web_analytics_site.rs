//! Provides a Cloudflare Web Analytics Site resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = web_analytics_site::create(
//!         "example",
//!         WebAnalyticsSiteArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .auto_install(true)
//!             .zone_tag("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/webAnalyticsSite:WebAnalyticsSite example <account_id>/<site_tag>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WebAnalyticsSiteArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether Cloudflare will automatically inject the JavaScript snippet for orange-clouded sites. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub auto_install: pulumi_wasm_rust::Output<bool>,
    /// The hostname to use for gray-clouded sites. Must provide only one of `zone_tag`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub host: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier for orange-clouded sites. Must provide only one of `host`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_tag: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct WebAnalyticsSiteResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether Cloudflare will automatically inject the JavaScript snippet for orange-clouded sites. **Modifying this attribute will force creation of a new resource.**
    pub auto_install: pulumi_wasm_rust::Output<bool>,
    /// The hostname to use for gray-clouded sites. Must provide only one of `zone_tag`. **Modifying this attribute will force creation of a new resource.**
    pub host: pulumi_wasm_rust::Output<Option<String>>,
    /// The ID for the ruleset associated to this Web Analytics Site.
    pub ruleset_id: pulumi_wasm_rust::Output<String>,
    /// The Web Analytics site tag.
    pub site_tag: pulumi_wasm_rust::Output<String>,
    /// The token for the Web Analytics site.
    pub site_token: pulumi_wasm_rust::Output<String>,
    /// The encoded JS snippet to add to your site's HTML page if auto_install is false.
    pub snippet: pulumi_wasm_rust::Output<String>,
    /// The zone identifier for orange-clouded sites. Must provide only one of `host`. **Modifying this attribute will force creation of a new resource.**
    pub zone_tag: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WebAnalyticsSiteArgs) -> WebAnalyticsSiteResult {

    let result = crate::bindings::pulumi::cloudflare::web_analytics_site::invoke(name, &crate::bindings::pulumi::cloudflare::web_analytics_site::Args {
        account_id: &args.account_id.get_inner(),
        auto_install: &args.auto_install.get_inner(),
        host: &args.host.get_inner(),
        zone_tag: &args.zone_tag.get_inner(),
    });

    WebAnalyticsSiteResult {
        account_id: crate::into_domain(result.account_id),
        auto_install: crate::into_domain(result.auto_install),
        host: crate::into_domain(result.host),
        ruleset_id: crate::into_domain(result.ruleset_id),
        site_tag: crate::into_domain(result.site_tag),
        site_token: crate::into_domain(result.site_token),
        snippet: crate::into_domain(result.snippet),
        zone_tag: crate::into_domain(result.zone_tag),
    }
}

//! Provides a resource to configure Bot Management.
//! 
//! Specifically, this resource can be used to manage:
//! 
//! - **Bot Fight Mode**
//! - **Super Bot Fight Mode**
//! - **Bot Management for Enterprise**
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = bot_management::create(
//!         "example",
//!         BotManagementArgs::builder()
//!             .enable_js(true)
//!             .optimize_wordpress(true)
//!             .sbfm_definitely_automated("block")
//!             .sbfm_likely_automated("managed_challenge")
//!             .sbfm_static_resource_protection(false)
//!             .sbfm_verified_bots("allow")
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
//! $ pulumi import cloudflare:index/botManagement:BotManagement example <zone_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct BotManagementArgs {
    /// Enable rule to block AI Scrapers and Crawlers.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ai_bots_protection: pulumi_wasm_rust::Output<Option<String>>,
    /// Automatically update to the newest bot detection models created by Cloudflare as they are released. [Learn more.](https://developers.cloudflare.com/bots/reference/machine-learning-models#model-versions-and-release-notes).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub auto_update_model: pulumi_wasm_rust::Output<Option<bool>>,
    /// Use lightweight, invisible JavaScript detections to improve Bot Management. [Learn more about JavaScript Detections](https://developers.cloudflare.com/bots/reference/javascript-detections/).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enable_js: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to enable Bot Fight Mode.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to optimize Super Bot Fight Mode protections for Wordpress.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub optimize_wordpress: pulumi_wasm_rust::Output<Option<bool>>,
    /// Super Bot Fight Mode (SBFM) action to take on definitely automated requests.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub sbfm_definitely_automated: pulumi_wasm_rust::Output<Option<String>>,
    /// Super Bot Fight Mode (SBFM) action to take on likely automated requests.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub sbfm_likely_automated: pulumi_wasm_rust::Output<Option<String>>,
    /// Super Bot Fight Mode (SBFM) to enable static resource protection. Enable if static resources on your application need bot protection. Note: Static resource protection can also result in legitimate traffic being blocked.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub sbfm_static_resource_protection: pulumi_wasm_rust::Output<Option<bool>>,
    /// Super Bot Fight Mode (SBFM) action to take on verified bots requests.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub sbfm_verified_bots: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to disable tracking the highest bot score for a session in the Bot Management cookie.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub suppress_session_score: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct BotManagementResult {
    /// Enable rule to block AI Scrapers and Crawlers.
    pub ai_bots_protection: pulumi_wasm_rust::Output<String>,
    /// Automatically update to the newest bot detection models created by Cloudflare as they are released. [Learn more.](https://developers.cloudflare.com/bots/reference/machine-learning-models#model-versions-and-release-notes).
    pub auto_update_model: pulumi_wasm_rust::Output<Option<bool>>,
    /// Use lightweight, invisible JavaScript detections to improve Bot Management. [Learn more about JavaScript Detections](https://developers.cloudflare.com/bots/reference/javascript-detections/).
    pub enable_js: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to enable Bot Fight Mode.
    pub fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether to optimize Super Bot Fight Mode protections for Wordpress.
    pub optimize_wordpress: pulumi_wasm_rust::Output<Option<bool>>,
    /// Super Bot Fight Mode (SBFM) action to take on definitely automated requests.
    pub sbfm_definitely_automated: pulumi_wasm_rust::Output<Option<String>>,
    /// Super Bot Fight Mode (SBFM) action to take on likely automated requests.
    pub sbfm_likely_automated: pulumi_wasm_rust::Output<Option<String>>,
    /// Super Bot Fight Mode (SBFM) to enable static resource protection. Enable if static resources on your application need bot protection. Note: Static resource protection can also result in legitimate traffic being blocked.
    pub sbfm_static_resource_protection: pulumi_wasm_rust::Output<Option<bool>>,
    /// Super Bot Fight Mode (SBFM) action to take on verified bots requests.
    pub sbfm_verified_bots: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to disable tracking the highest bot score for a session in the Bot Management cookie.
    pub suppress_session_score: pulumi_wasm_rust::Output<Option<bool>>,
    /// A read-only field that indicates whether the zone currently is running the latest ML model.
    pub using_latest_model: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: BotManagementArgs) -> BotManagementResult {

    let result = crate::bindings::pulumi::cloudflare::bot_management::invoke(name, &crate::bindings::pulumi::cloudflare::bot_management::Args {
        ai_bots_protection: &args.ai_bots_protection.get_inner(),
        auto_update_model: &args.auto_update_model.get_inner(),
        enable_js: &args.enable_js.get_inner(),
        fight_mode: &args.fight_mode.get_inner(),
        optimize_wordpress: &args.optimize_wordpress.get_inner(),
        sbfm_definitely_automated: &args.sbfm_definitely_automated.get_inner(),
        sbfm_likely_automated: &args.sbfm_likely_automated.get_inner(),
        sbfm_static_resource_protection: &args.sbfm_static_resource_protection.get_inner(),
        sbfm_verified_bots: &args.sbfm_verified_bots.get_inner(),
        suppress_session_score: &args.suppress_session_score.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    BotManagementResult {
        ai_bots_protection: crate::into_domain(result.ai_bots_protection),
        auto_update_model: crate::into_domain(result.auto_update_model),
        enable_js: crate::into_domain(result.enable_js),
        fight_mode: crate::into_domain(result.fight_mode),
        optimize_wordpress: crate::into_domain(result.optimize_wordpress),
        sbfm_definitely_automated: crate::into_domain(result.sbfm_definitely_automated),
        sbfm_likely_automated: crate::into_domain(result.sbfm_likely_automated),
        sbfm_static_resource_protection: crate::into_domain(result.sbfm_static_resource_protection),
        sbfm_verified_bots: crate::into_domain(result.sbfm_verified_bots),
        suppress_session_score: crate::into_domain(result.suppress_session_score),
        using_latest_model: crate::into_domain(result.using_latest_model),
        zone_id: crate::into_domain(result.zone_id),
    }
}

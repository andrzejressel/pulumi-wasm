//! Provides a Cloudflare Web Analytics Rule resource.
//! 
//! ## Example Usage
//! 
//! ```ignore
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
//!     let exampleWebAnalyticsRule = web_analytics_rule::create(
//!         "exampleWebAnalyticsRule",
//!         WebAnalyticsRuleArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .host("*")
//!             .inclusive(false)
//!             .is_paused(false)
//!             .paths(vec!["/excluded",])
//!             .ruleset_id("${example.rulesetId}")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/webAnalyticsRule:WebAnalyticsRule example <account_id>/<ruleset_id>/<rule_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WebAnalyticsRuleArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The host to apply the rule to.
    #[builder(into)]
    pub host: pulumi_wasm_rust::Output<String>,
    /// Whether the rule includes or excludes the matched traffic from being measured in Web Analytics.
    #[builder(into)]
    pub inclusive: pulumi_wasm_rust::Output<bool>,
    /// Whether the rule is paused or not.
    #[builder(into)]
    pub is_paused: pulumi_wasm_rust::Output<bool>,
    /// A list of paths to apply the rule to.
    #[builder(into)]
    pub paths: pulumi_wasm_rust::Output<Vec<String>>,
    /// The Web Analytics ruleset id. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub ruleset_id: pulumi_wasm_rust::Output<String>,
}

pub struct WebAnalyticsRuleResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The host to apply the rule to.
    pub host: pulumi_wasm_rust::Output<String>,
    /// Whether the rule includes or excludes the matched traffic from being measured in Web Analytics.
    pub inclusive: pulumi_wasm_rust::Output<bool>,
    /// Whether the rule is paused or not.
    pub is_paused: pulumi_wasm_rust::Output<bool>,
    /// A list of paths to apply the rule to.
    pub paths: pulumi_wasm_rust::Output<Vec<String>>,
    /// The Web Analytics ruleset id. **Modifying this attribute will force creation of a new resource.**
    pub ruleset_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WebAnalyticsRuleArgs) -> WebAnalyticsRuleResult {

    let result = crate::bindings::pulumi::cloudflare::web_analytics_rule::invoke(name, &crate::bindings::pulumi::cloudflare::web_analytics_rule::Args {
        account_id: &args.account_id.get_inner(),
        host: &args.host.get_inner(),
        inclusive: &args.inclusive.get_inner(),
        is_paused: &args.is_paused.get_inner(),
        paths: &args.paths.get_inner(),
        ruleset_id: &args.ruleset_id.get_inner(),
    });

    WebAnalyticsRuleResult {
        account_id: crate::into_domain(result.account_id),
        host: crate::into_domain(result.host),
        inclusive: crate::into_domain(result.inclusive),
        is_paused: crate::into_domain(result.is_paused),
        paths: crate::into_domain(result.paths),
        ruleset_id: crate::into_domain(result.ruleset_id),
    }
}


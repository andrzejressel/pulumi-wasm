//! Provides a resource to manage User Agent Blocking Rules.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example1 = user_agent_blocking_rule::create(
//!         "example1",
//!         UserAgentBlockingRuleArgs::builder()
//!             .configuration(
//!                 UserAgentBlockingRuleConfiguration::builder()
//!                     .target("ua")
//!                     .value("Chrome")
//!                     .build_struct(),
//!             )
//!             .description("My description 1")
//!             .mode("js_challenge")
//!             .paused(false)
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//!     let example2 = user_agent_blocking_rule::create(
//!         "example2",
//!         UserAgentBlockingRuleArgs::builder()
//!             .configuration(
//!                 UserAgentBlockingRuleConfiguration::builder()
//!                     .target("ua")
//!                     .value("Mozilla")
//!                     .build_struct(),
//!             )
//!             .description("My description 22")
//!             .mode("challenge")
//!             .paused(true)
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/userAgentBlockingRule:UserAgentBlockingRule example <zone_id>/<user_agent_blocking_rule_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct UserAgentBlockingRuleArgs {
    /// The configuration object for the current rule.
    #[builder(into)]
    pub configuration: pulumi_wasm_rust::Output<crate::types::UserAgentBlockingRuleConfiguration>,
    /// An informative summary of the rule.
    #[builder(into)]
    pub description: pulumi_wasm_rust::Output<String>,
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`.
    #[builder(into)]
    pub mode: pulumi_wasm_rust::Output<String>,
    /// When true, indicates that the rule is currently paused.
    #[builder(into)]
    pub paused: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct UserAgentBlockingRuleResult {
    /// The configuration object for the current rule.
    pub configuration: pulumi_wasm_rust::Output<crate::types::UserAgentBlockingRuleConfiguration>,
    /// An informative summary of the rule.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`.
    pub mode: pulumi_wasm_rust::Output<String>,
    /// When true, indicates that the rule is currently paused.
    pub paused: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: UserAgentBlockingRuleArgs) -> UserAgentBlockingRuleResult {

    let result = crate::bindings::pulumi::cloudflare::user_agent_blocking_rule::invoke(name, &crate::bindings::pulumi::cloudflare::user_agent_blocking_rule::Args {
        configuration: &args.configuration.get_inner(),
        description: &args.description.get_inner(),
        mode: &args.mode.get_inner(),
        paused: &args.paused.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    UserAgentBlockingRuleResult {
        configuration: crate::into_domain(result.configuration),
        description: crate::into_domain(result.description),
        mode: crate::into_domain(result.mode),
        paused: crate::into_domain(result.paused),
        zone_id: crate::into_domain(result.zone_id),
    }
}


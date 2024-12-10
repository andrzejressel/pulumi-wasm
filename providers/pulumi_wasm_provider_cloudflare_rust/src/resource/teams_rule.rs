//! Provides a Cloudflare Teams rule resource. Teams rules comprise secure web gateway policies.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = teams_rule::create(
//!         "example",
//!         TeamsRuleArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .action("block")
//!             .description("desc")
//!             .filters(vec!["http",])
//!             .name("office")
//!             .precedence(1)
//!             .rule_settings(
//!                 TeamsRuleRuleSettings::builder()
//!                     .blockPageEnabled(true)
//!                     .blockPageReason("access not permitted")
//!                     .build_struct(),
//!             )
//!             .traffic("http.request.uri == \"https://www.example.com/malicious\"")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/teamsRule:TeamsRule example <account_id>/<teams_rule_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct TeamsRuleArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`, `resolve`.
    #[builder(into)]
    pub action: pulumi_wasm_rust::Output<String>,
    /// The description of the teams rule.
    #[builder(into)]
    pub description: pulumi_wasm_rust::Output<String>,
    /// The wirefilter expression to be used for device_posture check matching.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub device_posture: pulumi_wasm_rust::Output<Option<String>>,
    /// Indicator of rule enablement.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The protocol or layer to evaluate the traffic and identity expressions.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The wirefilter expression to be used for identity matching.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub identity: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the teams rule.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The evaluation precedence of the teams rule.
    #[builder(into)]
    pub precedence: pulumi_wasm_rust::Output<i32>,
    /// Additional rule settings.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub rule_settings: pulumi_wasm_rust::Output<Option<crate::types::TeamsRuleRuleSettings>>,
    /// The wirefilter expression to be used for traffic matching.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub traffic: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct TeamsRuleResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`, `resolve`.
    pub action: pulumi_wasm_rust::Output<String>,
    /// The description of the teams rule.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The wirefilter expression to be used for device_posture check matching.
    pub device_posture: pulumi_wasm_rust::Output<String>,
    /// Indicator of rule enablement.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The protocol or layer to evaluate the traffic and identity expressions.
    pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The wirefilter expression to be used for identity matching.
    pub identity: pulumi_wasm_rust::Output<String>,
    /// The name of the teams rule.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The evaluation precedence of the teams rule.
    pub precedence: pulumi_wasm_rust::Output<i32>,
    /// Additional rule settings.
    pub rule_settings: pulumi_wasm_rust::Output<crate::types::TeamsRuleRuleSettings>,
    /// The wirefilter expression to be used for traffic matching.
    pub traffic: pulumi_wasm_rust::Output<String>,
    pub version: pulumi_wasm_rust::Output<i32>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TeamsRuleArgs) -> TeamsRuleResult {

    let result = crate::bindings::pulumi::cloudflare::teams_rule::invoke(name, &crate::bindings::pulumi::cloudflare::teams_rule::Args {
        account_id: &args.account_id.get_inner(),
        action: &args.action.get_inner(),
        description: &args.description.get_inner(),
        device_posture: &args.device_posture.get_inner(),
        enabled: &args.enabled.get_inner(),
        filters: &args.filters.get_inner(),
        identity: &args.identity.get_inner(),
        name: &args.name.get_inner(),
        precedence: &args.precedence.get_inner(),
        rule_settings: &args.rule_settings.get_inner(),
        traffic: &args.traffic.get_inner(),
    });

    TeamsRuleResult {
        account_id: crate::into_domain(result.account_id),
        action: crate::into_domain(result.action),
        description: crate::into_domain(result.description),
        device_posture: crate::into_domain(result.device_posture),
        enabled: crate::into_domain(result.enabled),
        filters: crate::into_domain(result.filters),
        identity: crate::into_domain(result.identity),
        name: crate::into_domain(result.name),
        precedence: crate::into_domain(result.precedence),
        rule_settings: crate::into_domain(result.rule_settings),
        traffic: crate::into_domain(result.traffic),
        version: crate::into_domain(result.version),
    }
}


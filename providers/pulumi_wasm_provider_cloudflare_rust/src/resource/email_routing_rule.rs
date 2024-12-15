//! The [Email Routing Rule](https://developers.cloudflare.com/email-routing/setup/email-routing-addresses/#email-rule-actions) resource allows you to create and manage email routing rules for a zone.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let main = email_routing_rule::create(
//!         "main",
//!         EmailRoutingRuleArgs::builder()
//!             .actions(
//!                 vec![
//!                     EmailRoutingRuleAction::builder(). type ("forward")
//!                     .values(vec!["destinationaddress@example.net",]).build_struct(),
//!                 ],
//!             )
//!             .enabled(true)
//!             .matchers(
//!                 vec![
//!                     EmailRoutingRuleMatcher::builder().field("to"). type ("literal")
//!                     .value("test@example.com").build_struct(),
//!                 ],
//!             )
//!             .name("terraform rule")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/emailRoutingRule:EmailRoutingRule example <zone_id>/<email_routing_rule_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingRuleArgs {
    /// Actions to take when a match is found.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub actions: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleAction>>>,
    /// Whether the email routing rule is enabled.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Matching patterns to forward to your actions.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub matchers: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleMatcher>>>,
    /// Routing rule name.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The priority of the email routing rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct EmailRoutingRuleResult {
    /// Actions to take when a match is found.
    pub actions: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleAction>>>,
    /// Whether the email routing rule is enabled.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Matching patterns to forward to your actions.
    pub matchers: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleMatcher>>>,
    /// Routing rule name.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The priority of the email routing rule.
    pub priority: pulumi_wasm_rust::Output<i32>,
    /// The tag of the email routing rule.
    pub tag: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: EmailRoutingRuleArgs) -> EmailRoutingRuleResult {

    let result = crate::bindings::pulumi::cloudflare::email_routing_rule::invoke(name, &crate::bindings::pulumi::cloudflare::email_routing_rule::Args {
        actions: &args.actions.get_inner(),
        enabled: &args.enabled.get_inner(),
        matchers: &args.matchers.get_inner(),
        name: &args.name.get_inner(),
        priority: &args.priority.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    EmailRoutingRuleResult {
        actions: crate::into_domain(result.actions),
        enabled: crate::into_domain(result.enabled),
        matchers: crate::into_domain(result.matchers),
        name: crate::into_domain(result.name),
        priority: crate::into_domain(result.priority),
        tag: crate::into_domain(result.tag),
        zone_id: crate::into_domain(result.zone_id),
    }
}

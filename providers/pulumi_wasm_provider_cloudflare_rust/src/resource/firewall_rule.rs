//! Define Firewall rules using filter expressions for more control over
//! how traffic is matched to the rule. A filter expression permits
//! selecting traffic by multiple criteria allowing greater freedom in
//! rule creation.
//! 
//! Filter expressions needs to be created first before using Firewall
//! Rule.
//! 
//! > `cloudflare.FirewallRule` is in a deprecation phase until January 15th, 2025.
//!   During this time period, this resource is still
//!   fully supported but you are strongly advised  to move to the
//!   `cloudflare.Ruleset` resource. Full details can be found in the
//!   developer documentation.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let wordpress = filter::create(
//!         "wordpress",
//!         FilterArgs::builder()
//!             .description("Wordpress break-in attempts that are outside of the office")
//!             .expression(
//!                 "(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1",
//!             )
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//!     let wordpressFirewallRule = firewall_rule::create(
//!         "wordpressFirewallRule",
//!         FirewallRuleArgs::builder()
//!             .action("block")
//!             .description("Block wordpress break-in attempts")
//!             .filter_id("${wordpress.id}")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/firewallRule:FirewallRule example <zone_id>/<firewall_rule_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct FirewallRuleArgs {
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `allow`, `js_challenge`, `managed_challenge`, `log`, `bypass`.
    #[builder(into)]
    pub action: pulumi_wasm_rust::Output<String>,
    /// A description of the rule to help identify it.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The identifier of the Filter to use for determining if the Firewall Rule should be triggered.
    #[builder(into)]
    pub filter_id: pulumi_wasm_rust::Output<String>,
    /// Whether this filter based firewall rule is currently paused.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// The priority of the rule to allow control of processing order. A lower number indicates high priority. If not provided, any rules with a priority will be sequenced before those without.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// List of products to bypass for a request when the bypass action is used. Available values: `zoneLockdown`, `uaBlock`, `bic`, `hot`, `securityLevel`, `rateLimit`, `waf`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub products: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct FirewallRuleResult {
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `allow`, `js_challenge`, `managed_challenge`, `log`, `bypass`.
    pub action: pulumi_wasm_rust::Output<String>,
    /// A description of the rule to help identify it.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The identifier of the Filter to use for determining if the Firewall Rule should be triggered.
    pub filter_id: pulumi_wasm_rust::Output<String>,
    /// Whether this filter based firewall rule is currently paused.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// The priority of the rule to allow control of processing order. A lower number indicates high priority. If not provided, any rules with a priority will be sequenced before those without.
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// List of products to bypass for a request when the bypass action is used. Available values: `zoneLockdown`, `uaBlock`, `bic`, `hot`, `securityLevel`, `rateLimit`, `waf`.
    pub products: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: FirewallRuleArgs) -> FirewallRuleResult {

    let result = crate::bindings::pulumi::cloudflare::firewall_rule::invoke(name, &crate::bindings::pulumi::cloudflare::firewall_rule::Args {
        action: &args.action.get_inner(),
        description: &args.description.get_inner(),
        filter_id: &args.filter_id.get_inner(),
        paused: &args.paused.get_inner(),
        priority: &args.priority.get_inner(),
        products: &args.products.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    FirewallRuleResult {
        action: crate::into_domain(result.action),
        description: crate::into_domain(result.description),
        filter_id: crate::into_domain(result.filter_id),
        paused: crate::into_domain(result.paused),
        priority: crate::into_domain(result.priority),
        products: crate::into_domain(result.products),
        zone_id: crate::into_domain(result.zone_id),
    }
}

//! Provides a Cloudflare IP Firewall Access Rule resource. Access
//! control can be applied on basis of IP addresses, IP ranges, AS
//! numbers or countries.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! configuration:
//!   # Allowlist office's network IP ranges on all account zones (or other lists of
//!   # resources).
//!   myOffice:
//!     type: list(string)
//!     default:
//!       - 192.0.2.0/24
//!       - 198.51.100.0/24
//!       - 2001:db8::/56
//! resources:
//!   # Challenge requests coming from known Tor exit nodes.
//!   torExitNodes:
//!     type: cloudflare:AccessRule
//!     name: tor_exit_nodes
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       notes: Requests coming from known Tor exit nodes
//!       mode: challenge
//!       configuration:
//!         target: country
//!         value: T1
//!   # Allowlist requests coming from Antarctica, but only for single zone.
//!   antarctica:
//!     type: cloudflare:AccessRule
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       notes: Requests coming from Antarctica
//!       mode: whitelist
//!       configuration:
//!         target: country
//!         value: AQ
//!   officeNetwork:
//!     type: cloudflare:AccessRule
//!     name: office_network
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       notes: Requests coming from office network
//!       mode: whitelist
//!       configuration:
//!         target: ip_range
//!         value:
//!           fn::select:
//!             - ${range.value}
//!             - ${myOffice}
//!     options: {}
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! User level access rule import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessRule:AccessRule default user/<user_id>/<rule_id>
//! ```
//! 
//! Zone level access rule import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessRule:AccessRule default zone/<zone_id>/<rule_id>
//! ```
//! 
//! Account level access rule import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessRule:AccessRule default account/<account_id>/<rule_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AccessRuleArgs {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Rule configuration to apply to a matched request. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub configuration: pulumi_wasm_rust::Output<crate::types::AccessRuleConfiguration>,
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`.
    #[builder(into)]
    pub mode: pulumi_wasm_rust::Output<String>,
    /// A personal note about the rule. Typically used as a reminder or explanation for the rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub notes: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessRuleResult {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Rule configuration to apply to a matched request. **Modifying this attribute will force creation of a new resource.**
    pub configuration: pulumi_wasm_rust::Output<crate::types::AccessRuleConfiguration>,
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`.
    pub mode: pulumi_wasm_rust::Output<String>,
    /// A personal note about the rule. Typically used as a reminder or explanation for the rule.
    pub notes: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessRuleArgs) -> AccessRuleResult {

    let result = crate::bindings::pulumi::cloudflare::access_rule::invoke(name, &crate::bindings::pulumi::cloudflare::access_rule::Args {
        account_id: &args.account_id.get_inner(),
        configuration: &args.configuration.get_inner(),
        mode: &args.mode.get_inner(),
        notes: &args.notes.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    AccessRuleResult {
        account_id: crate::into_domain(result.account_id),
        configuration: crate::into_domain(result.configuration),
        mode: crate::into_domain(result.mode),
        notes: crate::into_domain(result.notes),
        zone_id: crate::into_domain(result.zone_id),
    }
}

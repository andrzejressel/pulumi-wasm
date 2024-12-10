//! The [Cloudflare Ruleset Engine](https://developers.cloudflare.com/firewall/cf-rulesets)
//! allows you to create and deploy rules and rulesets.
//! 
//! The engine syntax, inspired by the Wireshark Display Filter language, is the
//! same syntax used in custom Firewall Rules. Cloudflare uses the Ruleset Engine
//! in different products, allowing you to configure several products using the same
//! basic syntax.
//! 
//! ## Import
//! 
//! Import an account scoped Ruleset configuration.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/ruleset:Ruleset example account/<account_id>/<ruleset_id>
//! ```
//! 
//! Import a zone scoped Ruleset configuration.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/ruleset:Ruleset example zone/<zone_id>/<ruleset_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct RulesetArgs {
    /// The account identifier to target for the resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Brief summary of the ruleset and its intended use.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `zone`.
    #[builder(into)]
    pub kind: pulumi_wasm_rust::Output<String>,
    /// Name of the ruleset.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
    #[builder(into)]
    pub phase: pulumi_wasm_rust::Output<String>,
    /// List of rules to apply to the ruleset.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::RulesetRule>>>,
    /// The zone identifier to target for the resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct RulesetResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Brief summary of the ruleset and its intended use.
    pub description: pulumi_wasm_rust::Output<String>,
    /// Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `zone`.
    pub kind: pulumi_wasm_rust::Output<String>,
    /// Name of the ruleset.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
    pub phase: pulumi_wasm_rust::Output<String>,
    /// List of rules to apply to the ruleset.
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::RulesetRule>>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RulesetArgs) -> RulesetResult {

    let result = crate::bindings::pulumi::cloudflare::ruleset::invoke(name, &crate::bindings::pulumi::cloudflare::ruleset::Args {
        account_id: &args.account_id.get_inner(),
        description: &args.description.get_inner(),
        kind: &args.kind.get_inner(),
        name: &args.name.get_inner(),
        phase: &args.phase.get_inner(),
        rules: &args.rules.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    RulesetResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
        phase: crate::into_domain(result.phase),
        rules: crate::into_domain(result.rules),
        zone_id: crate::into_domain(result.zone_id),
    }
}


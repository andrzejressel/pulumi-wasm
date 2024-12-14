//! Use this datasource to lookup Rulesets in an account or zone.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_rulesets::invoke(
//!         GetRulesetsArgs::builder()
//!             .filter(GetRulesetsFilter::builder().name(".*OWASP.*").build_struct())
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```

    #[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsArgs {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub filter: pulumi_wasm_rust::Output<Option<crate::types::GetRulesetsFilter>>,
    /// Include rule data in response.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub include_rules: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetRulesetsResult {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub filter: pulumi_wasm_rust::Output<Option<crate::types::GetRulesetsFilter>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Include rule data in response.
    pub include_rules: pulumi_wasm_rust::Output<Option<bool>>,
    pub rulesets: pulumi_wasm_rust::Output<Vec<crate::types::GetRulesetsRuleset>>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetRulesetsArgs
) -> GetRulesetsResult {

    let result = crate::bindings::pulumi::cloudflare::get_rulesets::invoke(
        &crate::bindings::pulumi::cloudflare::get_rulesets::Args {
                account_id: &args.account_id.get_inner(),
                filter: &args.filter.get_inner(),
                include_rules: &args.include_rules.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    GetRulesetsResult {
        account_id: crate::into_domain(result.account_id),
        filter: crate::into_domain(result.filter),
        id: crate::into_domain(result.id),
        include_rules: crate::into_domain(result.include_rules),
        rulesets: crate::into_domain(result.rulesets),
        zone_id: crate::into_domain(result.zone_id),
    }
}

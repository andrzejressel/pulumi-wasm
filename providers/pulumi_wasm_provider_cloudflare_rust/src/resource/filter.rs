//! Filter expressions that can be referenced across multiple features,
//! e.g. Firewall Rules. See [what is a filter](https://developers.cloudflare.com/firewall/api/cf-filters/what-is-a-filter/)
//! for more details and available fields and operators.
//! 
//! > `cloudflare.Filter` is in a deprecation phase until January 15th, 2025.
//!   During this time period, this resource is still fully
//!   supported but you are strongly advised to move to the
//!   `cloudflare.Ruleset` resource. Full details can be found in the
//!   developer documentation.
//! 
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! resources:
//!   wordpress:
//!     type: cloudflare:Filter
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       description: Wordpress break-in attempts that are outside of the office
//!       expression: (http.request.uri.path ~ ".*wp-login.php" or http.request.uri.path ~ ".*xmlrpc.php") and ip.src ne 192.0.2.1
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/filter:Filter example <zone_id>/<filter_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct FilterArgs {
    /// A note that you can use to describe the purpose of the filter.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The filter expression to be used.
    #[builder(into)]
    pub expression: pulumi_wasm_rust::Output<String>,
    /// Whether this filter is currently paused.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// Short reference tag to quickly select related rules.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ref_: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct FilterResult {
    /// A note that you can use to describe the purpose of the filter.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The filter expression to be used.
    pub expression: pulumi_wasm_rust::Output<String>,
    /// Whether this filter is currently paused.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// Short reference tag to quickly select related rules.
    pub ref_: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: FilterArgs) -> FilterResult {

    let result = crate::bindings::pulumi::cloudflare::filter::invoke(name, &crate::bindings::pulumi::cloudflare::filter::Args {
        description: &args.description.get_inner(),
        expression: &args.expression.get_inner(),
        paused: &args.paused.get_inner(),
        ref_: &args.ref_.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    FilterResult {
        description: crate::into_domain(result.description),
        expression: crate::into_domain(result.expression),
        paused: crate::into_domain(result.paused),
        ref_: crate::into_domain(result.ref_),
        zone_id: crate::into_domain(result.zone_id),
    }
}

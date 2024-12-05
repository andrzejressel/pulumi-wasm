//! A datasource to find Load Balancer Pools.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_load_balancer_pools::invoke(
//!         GetLoadBalancerPoolsArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .filter(
//!                 GetLoadBalancerPoolsFilter::builder()
//!                     .name("example-lb-pool")
//!                     .build_struct(),
//!             )
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetLoadBalancerPoolsArgs {
    /// The account identifier to target for the datasource lookups.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// One or more values used to look up Load Balancer pools. If more than one value is given all values must match in order to be included.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub filter: pulumi_wasm_rust::Output<Option<crate::types::GetLoadBalancerPoolsFilter>>,
    /// A list of Load Balancer Pools details.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::GetLoadBalancerPoolsPool>>>,
}

pub struct GetLoadBalancerPoolsResult {
    /// The account identifier to target for the datasource lookups.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// One or more values used to look up Load Balancer pools. If more than one value is given all values must match in order to be included.
    pub filter: pulumi_wasm_rust::Output<Option<crate::types::GetLoadBalancerPoolsFilter>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// A list of Load Balancer Pools details.
    pub pools: pulumi_wasm_rust::Output<Vec<crate::types::GetLoadBalancerPoolsPool>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetLoadBalancerPoolsArgs
) -> GetLoadBalancerPoolsResult {

    let result = crate::bindings::pulumi::cloudflare::get_load_balancer_pools::invoke(
        &crate::bindings::pulumi::cloudflare::get_load_balancer_pools::Args {
                account_id: &args.account_id.get_inner(),
                filter: &args.filter.get_inner(),
                pools: &args.pools.get_inner(),
        }
    );

    GetLoadBalancerPoolsResult {
        account_id: crate::into_domain(result.account_id),
        filter: crate::into_domain(result.filter),
        id: crate::into_domain(result.id),
        pools: crate::into_domain(result.pools),
    }
}

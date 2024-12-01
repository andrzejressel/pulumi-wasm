//! Provides a resource, that manages Cloudflare static routes for Magic
//! Transit or Magic WAN. Static routes are used to route traffic
//! through GRE tunnels.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:StaticRoute
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       description: New route for new prefix 192.0.2.0/24
//!       prefix: 192.0.2.0/24
//!       nexthop: 10.0.0.0
//!       priority: 100
//!       weight: 10
//!       coloNames:
//!         - den01
//!       coloRegions:
//!         - APAC
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/staticRoute:StaticRoute example <account_id>/<static_route_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct StaticRouteArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// List of Cloudflare colocation regions for this static route.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub colo_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of Cloudflare colocation names for this static route.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub colo_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Description of the static route.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The nexthop IP address where traffic will be routed to.
    #[builder(into)]
    pub nexthop: pulumi_wasm_rust::Output<String>,
    /// Your network prefix using CIDR notation.
    #[builder(into)]
    pub prefix: pulumi_wasm_rust::Output<String>,
    /// The priority for the static route.
    #[builder(into)]
    pub priority: pulumi_wasm_rust::Output<i32>,
    /// The optional weight for ECMP routes. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub weight: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct StaticRouteResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// List of Cloudflare colocation regions for this static route.
    pub colo_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of Cloudflare colocation names for this static route.
    pub colo_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Description of the static route.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The nexthop IP address where traffic will be routed to.
    pub nexthop: pulumi_wasm_rust::Output<String>,
    /// Your network prefix using CIDR notation.
    pub prefix: pulumi_wasm_rust::Output<String>,
    /// The priority for the static route.
    pub priority: pulumi_wasm_rust::Output<i32>,
    /// The optional weight for ECMP routes. **Modifying this attribute will force creation of a new resource.**
    pub weight: pulumi_wasm_rust::Output<Option<i32>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: StaticRouteArgs) -> StaticRouteResult {

    let result = crate::bindings::pulumi::cloudflare::static_route::invoke(name, &crate::bindings::pulumi::cloudflare::static_route::Args {
        account_id: &args.account_id.get_inner(),
        colo_names: &args.colo_names.get_inner(),
        colo_regions: &args.colo_regions.get_inner(),
        description: &args.description.get_inner(),
        nexthop: &args.nexthop.get_inner(),
        prefix: &args.prefix.get_inner(),
        priority: &args.priority.get_inner(),
        weight: &args.weight.get_inner(),
    });

    StaticRouteResult {
        account_id: crate::into_domain(result.account_id),
        colo_names: crate::into_domain(result.colo_names),
        colo_regions: crate::into_domain(result.colo_regions),
        description: crate::into_domain(result.description),
        nexthop: crate::into_domain(result.nexthop),
        prefix: crate::into_domain(result.prefix),
        priority: crate::into_domain(result.priority),
        weight: crate::into_domain(result.weight),
    }
}

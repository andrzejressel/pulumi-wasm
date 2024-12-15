//! Provides a Cloudflare Load Balancer pool resource. This provides a
//! pool of origins that can be used by a Cloudflare Load Balancer.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = load_balancer_pool::create(
//!         "example",
//!         LoadBalancerPoolArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .description("example load balancer pool")
//!             .enabled(false)
//!             .latitude(55)
//!             .load_sheddings(
//!                 vec![
//!                     LoadBalancerPoolLoadShedding::builder().defaultPercent(55)
//!                     .defaultPolicy("random").sessionPercent(12).sessionPolicy("hash")
//!                     .build_struct(),
//!                 ],
//!             )
//!             .longitude(-12)
//!             .minimum_origins(1)
//!             .name("example-pool")
//!             .notification_email("someone@example.com")
//!             .origin_steerings(
//!                 vec![
//!                     LoadBalancerPoolOriginSteering::builder().policy("random")
//!                     .build_struct(),
//!                 ],
//!             )
//!             .origins(
//!                 vec![
//!                     LoadBalancerPoolOrigin::builder().address("192.0.2.1").enabled(false)
//!                     .headers(vec![LoadBalancerPoolOriginHeader::builder().header("Host")
//!                     .values(vec!["example-1",]).build_struct(),]).name("example-1")
//!                     .build_struct(), LoadBalancerPoolOrigin::builder()
//!                     .address("192.0.2.2")
//!                     .headers(vec![LoadBalancerPoolOriginHeader::builder().header("Host")
//!                     .values(vec!["example-2",]).build_struct(),]).name("example-2")
//!                     .build_struct(),
//!                 ],
//!             )
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/loadBalancerPool:LoadBalancerPool example <account_id>/<load_balancer_pool_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct LoadBalancerPoolArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub check_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Free text description.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to enable (the default) this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any). Defaults to `true`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The latitude this pool is physically located at; used for proximity steering.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub latitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// Setting for controlling load shedding for this pool.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub load_sheddings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolLoadShedding>>>,
    /// The longitude this pool is physically located at; used for proximity steering.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub longitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// The minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and we will failover to the next available pool. Defaults to `1`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub minimum_origins: pulumi_wasm_rust::Output<Option<i32>>,
    /// The ID of the Monitor to use for health checking origins within this pool.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub monitor: pulumi_wasm_rust::Output<Option<String>>,
    /// A short name (tag) for the pool.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub notification_email: pulumi_wasm_rust::Output<Option<String>>,
    /// Set an origin steering policy to control origin selection within a pool.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub origin_steerings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolOriginSteering>>>,
    /// The list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy.
    #[builder(into)]
    pub origins: pulumi_wasm_rust::Output<Vec<crate::types::LoadBalancerPoolOrigin>>,
}

pub struct LoadBalancerPoolResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api).
    pub check_regions: pulumi_wasm_rust::Output<Vec<String>>,
    /// The RFC3339 timestamp of when the load balancer was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// Free text description.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to enable (the default) this pool. Disabled pools will not receive traffic and are excluded from health checks. Disabling a pool will cause any load balancers using it to failover to the next pool (if any). Defaults to `true`.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The latitude this pool is physically located at; used for proximity steering.
    pub latitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// Setting for controlling load shedding for this pool.
    pub load_sheddings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolLoadShedding>>>,
    /// The longitude this pool is physically located at; used for proximity steering.
    pub longitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// The minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and we will failover to the next available pool. Defaults to `1`.
    pub minimum_origins: pulumi_wasm_rust::Output<Option<i32>>,
    /// The RFC3339 timestamp of when the load balancer was last modified.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// The ID of the Monitor to use for health checking origins within this pool.
    pub monitor: pulumi_wasm_rust::Output<Option<String>>,
    /// A short name (tag) for the pool.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list.
    pub notification_email: pulumi_wasm_rust::Output<Option<String>>,
    /// Set an origin steering policy to control origin selection within a pool.
    pub origin_steerings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolOriginSteering>>>,
    /// The list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy.
    pub origins: pulumi_wasm_rust::Output<Vec<crate::types::LoadBalancerPoolOrigin>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: LoadBalancerPoolArgs) -> LoadBalancerPoolResult {

    let result = crate::bindings::pulumi::cloudflare::load_balancer_pool::invoke(name, &crate::bindings::pulumi::cloudflare::load_balancer_pool::Args {
        account_id: &args.account_id.get_inner(),
        check_regions: &args.check_regions.get_inner(),
        description: &args.description.get_inner(),
        enabled: &args.enabled.get_inner(),
        latitude: &args.latitude.get_inner(),
        load_sheddings: &args.load_sheddings.get_inner(),
        longitude: &args.longitude.get_inner(),
        minimum_origins: &args.minimum_origins.get_inner(),
        monitor: &args.monitor.get_inner(),
        name: &args.name.get_inner(),
        notification_email: &args.notification_email.get_inner(),
        origin_steerings: &args.origin_steerings.get_inner(),
        origins: &args.origins.get_inner(),
    });

    LoadBalancerPoolResult {
        account_id: crate::into_domain(result.account_id),
        check_regions: crate::into_domain(result.check_regions),
        created_on: crate::into_domain(result.created_on),
        description: crate::into_domain(result.description),
        enabled: crate::into_domain(result.enabled),
        latitude: crate::into_domain(result.latitude),
        load_sheddings: crate::into_domain(result.load_sheddings),
        longitude: crate::into_domain(result.longitude),
        minimum_origins: crate::into_domain(result.minimum_origins),
        modified_on: crate::into_domain(result.modified_on),
        monitor: crate::into_domain(result.monitor),
        name: crate::into_domain(result.name),
        notification_email: crate::into_domain(result.notification_email),
        origin_steerings: crate::into_domain(result.origin_steerings),
        origins: crate::into_domain(result.origins),
    }
}

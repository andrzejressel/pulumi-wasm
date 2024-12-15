//! Provides a Cloudflare Zone Lockdown resource. Zone Lockdown allows
//! you to define one or more URLs (with wildcard matching on the domain
//! or path) that will only permit access if the request originates
//! from an IP address that matches a safelist of one or more IP
//! addresses and/or IP ranges.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   # Restrict access to these endpoints to requests from a known IP address range.
//!   example:
//!     type: cloudflare:ZoneLockdown
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       paused: 'false'
//!       description: Restrict access to these endpoints to requests from a known IP address range
//!       urls:
//!         - api.mysite.com/some/endpoint*
//!       configurations:
//!         - target: ip_range
//!           value: 192.0.2.0/24
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zoneLockdown:ZoneLockdown example <zone_id>/<lockdown_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZoneLockdownArgs {
    /// A list of IP addresses or IP ranges to match the request against specified in target, value pairs.
    #[builder(into)]
    pub configurations: pulumi_wasm_rust::Output<Vec<crate::types::ZoneLockdownConfiguration>>,
    /// A description about the lockdown entry. Typically used as a reminder or explanation for the lockdown.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Boolean of whether this zone lockdown is currently paused. Defaults to `false`.
    #[builder(into, default)]
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    #[builder(into, default)]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// A list of simple wildcard patterns to match requests against. The order of the urls is unimportant.
    #[builder(into)]
    pub urls: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneLockdownResult {
    /// A list of IP addresses or IP ranges to match the request against specified in target, value pairs.
    pub configurations: pulumi_wasm_rust::Output<Vec<crate::types::ZoneLockdownConfiguration>>,
    /// A description about the lockdown entry. Typically used as a reminder or explanation for the lockdown.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Boolean of whether this zone lockdown is currently paused. Defaults to `false`.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// A list of simple wildcard patterns to match requests against. The order of the urls is unimportant.
    pub urls: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneLockdownArgs) -> ZoneLockdownResult {

    let result = crate::bindings::pulumi::cloudflare::zone_lockdown::invoke(name, &crate::bindings::pulumi::cloudflare::zone_lockdown::Args {
        configurations: &args.configurations.get_inner(),
        description: &args.description.get_inner(),
        paused: &args.paused.get_inner(),
        priority: &args.priority.get_inner(),
        urls: &args.urls.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZoneLockdownResult {
        configurations: crate::into_domain(result.configurations),
        description: crate::into_domain(result.description),
        paused: crate::into_domain(result.paused),
        priority: crate::into_domain(result.priority),
        urls: crate::into_domain(result.urls),
        zone_id: crate::into_domain(result.zone_id),
    }
}

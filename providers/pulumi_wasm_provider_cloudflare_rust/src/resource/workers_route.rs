//! Provides a Cloudflare worker route resource. A route will also require a `cloudflare.WorkerScript`.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let myRoute = workers_route::create(
//!         "myRoute",
//!         WorkersRouteArgs::builder()
//!             .pattern("example.com/*")
//!             .script_name("${myScript.name}")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//!     let myScript = workers_script::create(
//!         "myScript",
//!         WorkersScriptArgs::builder().build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersRoute:WorkersRoute example <zone_id>/<route_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WorkersRouteArgs {
    /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
    #[builder(into)]
    pub pattern: pulumi_wasm_rust::Output<String>,
    /// Worker script name to invoke for requests that match the route pattern.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub script_name: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersRouteResult {
    /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
    pub pattern: pulumi_wasm_rust::Output<String>,
    /// Worker script name to invoke for requests that match the route pattern.
    pub script_name: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersRouteArgs) -> WorkersRouteResult {

    let result = crate::bindings::pulumi::cloudflare::workers_route::invoke(name, &crate::bindings::pulumi::cloudflare::workers_route::Args {
        pattern: &args.pattern.get_inner(),
        script_name: &args.script_name.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    WorkersRouteResult {
        pattern: crate::into_domain(result.pattern),
        script_name: crate::into_domain(result.script_name),
        zone_id: crate::into_domain(result.zone_id),
    }
}

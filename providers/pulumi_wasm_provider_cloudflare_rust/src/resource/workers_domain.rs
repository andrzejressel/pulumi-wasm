//! Creates a Worker Custom Domain.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = workers_domain::create(
//!         "example",
//!         WorkersDomainArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .hostname("subdomain.example.com")
//!             .service("my-service")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersDomain:WorkersDomain example <account_id>/<worker_domain_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkersDomainArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker environment. Defaults to `production`.
    #[builder(into, default)]
    pub environment: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname of the Worker Domain.
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Name of worker script to attach the domain to.
    #[builder(into)]
    pub service: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersDomainResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker environment. Defaults to `production`.
    pub environment: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname of the Worker Domain.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Name of worker script to attach the domain to.
    pub service: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersDomainArgs) -> WorkersDomainResult {

    let result = crate::bindings::pulumi::cloudflare::workers_domain::invoke(name, &crate::bindings::pulumi::cloudflare::workers_domain::Args {
        account_id: &args.account_id.get_inner(),
        environment: &args.environment.get_inner(),
        hostname: &args.hostname.get_inner(),
        service: &args.service.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    WorkersDomainResult {
        account_id: crate::into_domain(result.account_id),
        environment: crate::into_domain(result.environment),
        hostname: crate::into_domain(result.hostname),
        service: crate::into_domain(result.service),
        zone_id: crate::into_domain(result.zone_id),
    }
}

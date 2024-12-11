//! Allows management of the Logpull Retention settings used to control whether or not to retain HTTP request logs.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:LogpullRetention
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       enabled: 'true'
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/logpullRetention:LogpullRetention example <zone_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct LogpullRetentionArgs {
    /// Whether you wish to retain logs or not.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct LogpullRetentionResult {
    /// Whether you wish to retain logs or not.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: LogpullRetentionArgs) -> LogpullRetentionResult {

    let result = crate::bindings::pulumi::cloudflare::logpull_retention::invoke(name, &crate::bindings::pulumi::cloudflare::logpull_retention::Args {
        enabled: &args.enabled.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    LogpullRetentionResult {
        enabled: crate::into_domain(result.enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}

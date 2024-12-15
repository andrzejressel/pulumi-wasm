//! Provides a Cloudflare Teams Proxy Endpoint resource. Teams Proxy
//! Endpoints are used for pointing proxy clients at Cloudflare Secure
//! Gateway.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = teams_proxy_endpoint::create(
//!         "example",
//!         TeamsProxyEndpointArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .ips(vec!["192.0.2.0/24",])
//!             .name("office")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/teamsProxyEndpoint:TeamsProxyEndpoint example <account_id>/<proxy_endpoint_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsProxyEndpointArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The networks CIDRs that will be allowed to initiate proxy connections.
    #[builder(into)]
    pub ips: pulumi_wasm_rust::Output<Vec<String>>,
    /// Name of the teams proxy endpoint.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct TeamsProxyEndpointResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The networks CIDRs that will be allowed to initiate proxy connections.
    pub ips: pulumi_wasm_rust::Output<Vec<String>>,
    /// Name of the teams proxy endpoint.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The FQDN that proxy clients should be pointed at.
    pub subdomain: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TeamsProxyEndpointArgs) -> TeamsProxyEndpointResult {

    let result = crate::bindings::pulumi::cloudflare::teams_proxy_endpoint::invoke(name, &crate::bindings::pulumi::cloudflare::teams_proxy_endpoint::Args {
        account_id: &args.account_id.get_inner(),
        ips: &args.ips.get_inner(),
        name: &args.name.get_inner(),
    });

    TeamsProxyEndpointResult {
        account_id: crate::into_domain(result.account_id),
        ips: crate::into_domain(result.ips),
        name: crate::into_domain(result.name),
        subdomain: crate::into_domain(result.subdomain),
    }
}

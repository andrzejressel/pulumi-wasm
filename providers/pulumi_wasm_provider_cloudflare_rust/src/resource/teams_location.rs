//! Provides a Cloudflare Teams Location resource. Teams Locations are
//! referenced when creating secure web gateway policies.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = teams_location::create(
//!         "example",
//!         TeamsLocationArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .client_default(true)
//!             .ecs_support(false)
//!             .name("office")
//!             .networks(
//!                 vec![
//!                     TeamsLocationNetwork::builder().network("203.0.113.1/32")
//!                     .build_struct(), TeamsLocationNetwork::builder()
//!                     .network("203.0.113.2/32").build_struct(),
//!                 ],
//!             )
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/teamsLocation:TeamsLocation example <account_id>/<teams_location_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct TeamsLocationArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Indicator that this is the default location.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub client_default: pulumi_wasm_rust::Output<Option<bool>>,
    /// Indicator that this location needs to resolve EDNS queries.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ecs_support: pulumi_wasm_rust::Output<Option<bool>>,
    /// Name of the teams location.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The networks CIDRs that comprise the location.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub networks: pulumi_wasm_rust::Output<Option<Vec<crate::types::TeamsLocationNetwork>>>,
}

pub struct TeamsLocationResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Indicator that anonymized logs are enabled.
    pub anonymized_logs_enabled: pulumi_wasm_rust::Output<bool>,
    /// Indicator that this is the default location.
    pub client_default: pulumi_wasm_rust::Output<Option<bool>>,
    /// The FQDN that DoH clients should be pointed at.
    pub doh_subdomain: pulumi_wasm_rust::Output<String>,
    /// Indicator that this location needs to resolve EDNS queries.
    pub ecs_support: pulumi_wasm_rust::Output<Option<bool>>,
    /// Client IP address.
    pub ip: pulumi_wasm_rust::Output<String>,
    /// IP to direct all IPv4 DNS queries to.
    pub ipv4_destination: pulumi_wasm_rust::Output<String>,
    /// Name of the teams location.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The networks CIDRs that comprise the location.
    pub networks: pulumi_wasm_rust::Output<Option<Vec<crate::types::TeamsLocationNetwork>>>,
    pub policy_ids: pulumi_wasm_rust::Output<Vec<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TeamsLocationArgs) -> TeamsLocationResult {

    let result = crate::bindings::pulumi::cloudflare::teams_location::invoke(name, &crate::bindings::pulumi::cloudflare::teams_location::Args {
        account_id: &args.account_id.get_inner(),
        client_default: &args.client_default.get_inner(),
        ecs_support: &args.ecs_support.get_inner(),
        name: &args.name.get_inner(),
        networks: &args.networks.get_inner(),
    });

    TeamsLocationResult {
        account_id: crate::into_domain(result.account_id),
        anonymized_logs_enabled: crate::into_domain(result.anonymized_logs_enabled),
        client_default: crate::into_domain(result.client_default),
        doh_subdomain: crate::into_domain(result.doh_subdomain),
        ecs_support: crate::into_domain(result.ecs_support),
        ip: crate::into_domain(result.ip),
        ipv4_destination: crate::into_domain(result.ipv4_destination),
        name: crate::into_domain(result.name),
        networks: crate::into_domain(result.networks),
        policy_ids: crate::into_domain(result.policy_ids),
    }
}

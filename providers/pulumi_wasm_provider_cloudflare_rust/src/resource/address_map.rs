//! Provides the ability to manage IP addresses that can be used by DNS records when
//! they are proxied through Cloudflare.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = address_map::create(
//!         "example",
//!         AddressMapArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .default_sni("*.example.com")
//!             .description("My address map")
//!             .enabled(true)
//!             .ips(
//!                 vec![
//!                     AddressMapIp::builder().ip("192.0.2.1").build_struct(),
//!                     AddressMapIp::builder().ip("203.0.113.1").build_struct(),
//!                 ],
//!             )
//!             .memberships(
//!                 vec![
//!                     AddressMapMembership::builder()
//!                     .identifier("92f17202ed8bd63d69a66b86a49a8f6b").kind("account")
//!                     .build_struct(), AddressMapMembership::builder()
//!                     .identifier("023e105f4ecef8ad9ca31a8372d0c353").kind("zone")
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
//! $ pulumi import cloudflare:index/addressMap:AddressMap example <account_id>/<address_map_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AddressMapArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub default_sni: pulumi_wasm_rust::Output<Option<String>>,
    /// Description of the address map.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether the Address Map is enabled or not.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The set of IPs on the Address Map.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ips: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapIp>>>,
    /// Zones and Accounts which will be assigned IPs on this Address Map.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub memberships: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapMembership>>>,
}

pub struct AddressMapResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If set to false, then the Address Map cannot be deleted via API. This is true for Cloudflare-managed maps.
    pub can_delete: pulumi_wasm_rust::Output<bool>,
    /// If set to false, then the IPs on the Address Map cannot be modified via the API. This is true for Cloudflare-managed maps.
    pub can_modify_ips: pulumi_wasm_rust::Output<bool>,
    /// If you have legacy TLS clients which do not send the TLS server name indicator, then you can specify one default SNI on the map.
    pub default_sni: pulumi_wasm_rust::Output<Option<String>>,
    /// Description of the address map.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether the Address Map is enabled or not.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The set of IPs on the Address Map.
    pub ips: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapIp>>>,
    /// Zones and Accounts which will be assigned IPs on this Address Map.
    pub memberships: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapMembership>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AddressMapArgs) -> AddressMapResult {

    let result = crate::bindings::pulumi::cloudflare::address_map::invoke(name, &crate::bindings::pulumi::cloudflare::address_map::Args {
        account_id: &args.account_id.get_inner(),
        default_sni: &args.default_sni.get_inner(),
        description: &args.description.get_inner(),
        enabled: &args.enabled.get_inner(),
        ips: &args.ips.get_inner(),
        memberships: &args.memberships.get_inner(),
    });

    AddressMapResult {
        account_id: crate::into_domain(result.account_id),
        can_delete: crate::into_domain(result.can_delete),
        can_modify_ips: crate::into_domain(result.can_modify_ips),
        default_sni: crate::into_domain(result.default_sni),
        description: crate::into_domain(result.description),
        enabled: crate::into_domain(result.enabled),
        ips: crate::into_domain(result.ips),
        memberships: crate::into_domain(result.memberships),
    }
}

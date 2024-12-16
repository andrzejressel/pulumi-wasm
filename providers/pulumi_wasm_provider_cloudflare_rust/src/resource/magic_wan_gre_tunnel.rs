//! Provides a resource, that manages GRE tunnels for Magic Transit.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = magic_wan_gre_tunnel::create(
//!         "example",
//!         MagicWanGreTunnelArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .cloudflare_gre_endpoint("203.0.113.2")
//!             .customer_gre_endpoint("203.0.113.1")
//!             .description("Tunnel for ISP X")
//!             .health_check_enabled(true)
//!             .health_check_target("203.0.113.1")
//!             .health_check_type("reply")
//!             .interface_address("192.0.2.0/31")
//!             .mtu(1476)
//!             .name("GRE_1")
//!             .ttl(64)
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/magicWanGreTunnel:MagicWanGreTunnel example <account_id>/<tunnel_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct MagicWanGreTunnelArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The IP address assigned to the Cloudflare side of the GRE tunnel.
    #[builder(into)]
    pub cloudflare_gre_endpoint: pulumi_wasm_rust::Output<String>,
    /// The IP address assigned to the customer side of the GRE tunnel.
    #[builder(into)]
    pub customer_gre_endpoint: pulumi_wasm_rust::Output<String>,
    /// Description of the GRE tunnel intent.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies if ICMP tunnel health checks are enabled.
    #[builder(into, default)]
    pub health_check_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The IP address of the customer endpoint that will receive tunnel health checks.
    #[builder(into, default)]
    pub health_check_target: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies the ICMP echo type for the health check. Available values: `request`, `reply`.
    #[builder(into, default)]
    pub health_check_type: pulumi_wasm_rust::Output<Option<String>>,
    /// 31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel.
    #[builder(into)]
    pub interface_address: pulumi_wasm_rust::Output<String>,
    /// Maximum Transmission Unit (MTU) in bytes for the GRE tunnel.
    #[builder(into, default)]
    pub mtu: pulumi_wasm_rust::Output<Option<i32>>,
    /// Name of the GRE tunnel.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Time To Live (TTL) in number of hops of the GRE tunnel.
    #[builder(into, default)]
    pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct MagicWanGreTunnelResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The IP address assigned to the Cloudflare side of the GRE tunnel.
    pub cloudflare_gre_endpoint: pulumi_wasm_rust::Output<String>,
    /// The IP address assigned to the customer side of the GRE tunnel.
    pub customer_gre_endpoint: pulumi_wasm_rust::Output<String>,
    /// Description of the GRE tunnel intent.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Specifies if ICMP tunnel health checks are enabled.
    pub health_check_enabled: pulumi_wasm_rust::Output<bool>,
    /// The IP address of the customer endpoint that will receive tunnel health checks.
    pub health_check_target: pulumi_wasm_rust::Output<String>,
    /// Specifies the ICMP echo type for the health check. Available values: `request`, `reply`.
    pub health_check_type: pulumi_wasm_rust::Output<String>,
    /// 31-bit prefix (/31 in CIDR notation) supporting 2 hosts, one for each side of the tunnel.
    pub interface_address: pulumi_wasm_rust::Output<String>,
    /// Maximum Transmission Unit (MTU) in bytes for the GRE tunnel.
    pub mtu: pulumi_wasm_rust::Output<i32>,
    /// Name of the GRE tunnel.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Time To Live (TTL) in number of hops of the GRE tunnel.
    pub ttl: pulumi_wasm_rust::Output<i32>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: MagicWanGreTunnelArgs) -> MagicWanGreTunnelResult {

    let result = crate::bindings::pulumi::cloudflare::magic_wan_gre_tunnel::invoke(name, &crate::bindings::pulumi::cloudflare::magic_wan_gre_tunnel::Args {
        account_id: &args.account_id.get_inner(),
        cloudflare_gre_endpoint: &args.cloudflare_gre_endpoint.get_inner(),
        customer_gre_endpoint: &args.customer_gre_endpoint.get_inner(),
        description: &args.description.get_inner(),
        health_check_enabled: &args.health_check_enabled.get_inner(),
        health_check_target: &args.health_check_target.get_inner(),
        health_check_type: &args.health_check_type.get_inner(),
        interface_address: &args.interface_address.get_inner(),
        mtu: &args.mtu.get_inner(),
        name: &args.name.get_inner(),
        ttl: &args.ttl.get_inner(),
    });

    MagicWanGreTunnelResult {
        account_id: crate::into_domain(result.account_id),
        cloudflare_gre_endpoint: crate::into_domain(result.cloudflare_gre_endpoint),
        customer_gre_endpoint: crate::into_domain(result.customer_gre_endpoint),
        description: crate::into_domain(result.description),
        health_check_enabled: crate::into_domain(result.health_check_enabled),
        health_check_target: crate::into_domain(result.health_check_target),
        health_check_type: crate::into_domain(result.health_check_type),
        interface_address: crate::into_domain(result.interface_address),
        mtu: crate::into_domain(result.mtu),
        name: crate::into_domain(result.name),
        ttl: crate::into_domain(result.ttl),
    }
}

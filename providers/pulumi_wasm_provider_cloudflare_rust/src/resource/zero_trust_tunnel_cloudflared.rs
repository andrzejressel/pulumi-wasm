//! Tunnel exposes applications running on your local web server on any
//! network with an internet connection without manually adding DNS
//! records or configuring a firewall or router.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = zero_trust_tunnel_cloudflared::create(
//!         "example",
//!         ZeroTrustTunnelCloudflaredArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .name("my-tunnel")
//!             .secret("AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustTunnelCloudflared:ZeroTrustTunnelCloudflared example <account_id>/<tunnel_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustTunnelCloudflaredArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Indicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard or using tunnel*config, tunnel*route or tunnel*virtual*network resources. Available values: `local`, `cloudflare`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub config_src: pulumi_wasm_rust::Output<Option<String>>,
    /// A user-friendly name chosen when the tunnel is created. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// 32 or more bytes, encoded as a base64 string. The Create Argo Tunnel endpoint sets this as the tunnel's password. Anyone wishing to run the tunnel needs this password. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub secret: pulumi_wasm_rust::Output<String>,
}

pub struct ZeroTrustTunnelCloudflaredResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Usable CNAME for accessing the Tunnel.
    pub cname: pulumi_wasm_rust::Output<String>,
    /// Indicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard or using tunnel*config, tunnel*route or tunnel*virtual*network resources. Available values: `local`, `cloudflare`. **Modifying this attribute will force creation of a new resource.**
    pub config_src: pulumi_wasm_rust::Output<Option<String>>,
    /// A user-friendly name chosen when the tunnel is created. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// 32 or more bytes, encoded as a base64 string. The Create Argo Tunnel endpoint sets this as the tunnel's password. Anyone wishing to run the tunnel needs this password. **Modifying this attribute will force creation of a new resource.**
    pub secret: pulumi_wasm_rust::Output<String>,
    /// Token used by a connector to authenticate and run the tunnel.
    pub tunnel_token: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ZeroTrustTunnelCloudflaredArgs
) -> ZeroTrustTunnelCloudflaredResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_tunnel_cloudflared::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zero_trust_tunnel_cloudflared::Args {
                account_id: &args.account_id.get_inner(),
                config_src: &args.config_src.get_inner(),
                name: &args.name.get_inner(),
                secret: &args.secret.get_inner(),
        }
    );

    ZeroTrustTunnelCloudflaredResult {
        account_id: crate::into_domain(result.account_id),
        cname: crate::into_domain(result.cname),
        config_src: crate::into_domain(result.config_src),
        name: crate::into_domain(result.name),
        secret: crate::into_domain(result.secret),
        tunnel_token: crate::into_domain(result.tunnel_token),
    }
}

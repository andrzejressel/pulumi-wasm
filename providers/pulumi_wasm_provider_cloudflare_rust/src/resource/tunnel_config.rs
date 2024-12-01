//! Provides a Cloudflare Tunnel configuration resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! resources:
//!   exampleTunnel:
//!     type: cloudflare:ZeroTrustTunnelCloudflared
//!     name: example_tunnel
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: example_tunnel
//!       secret: <32 character secret>
//!   exampleConfig:
//!     type: cloudflare:TunnelConfig
//!     name: example_config
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       tunnelId: ${exampleTunnel.id}
//!       config:
//!         warpRouting:
//!           enabled: true
//!         originRequest:
//!           connectTimeout: 1m0s
//!           tlsTimeout: 1m0s
//!           tcpKeepAlive: 1m0s
//!           noHappyEyeballs: false
//!           keepAliveConnections: 1024
//!           keepAliveTimeout: 1m0s
//!           httpHostHeader: baz
//!           originServerName: foobar
//!           caPool: /path/to/unsigned/ca/pool
//!           noTlsVerify: false
//!           disableChunkedEncoding: false
//!           bastionMode: false
//!           proxyAddress: 10.0.0.1
//!           proxyPort: '8123'
//!           proxyType: socks
//!           ipRules:
//!             - prefix: /web
//!               ports:
//!                 - 80
//!                 - 443
//!               allow: false
//!         ingressRules:
//!           - hostname: foo
//!             path: /bar
//!             service: http://10.0.0.2:8080
//!             originRequest:
//!               connectTimeout: 2m0s
//!               access:
//!                 required: true
//!                 teamName: terraform
//!                 audTags:
//!                   - AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
//!           - service: https://10.0.0.3:8081
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/tunnelConfig:TunnelConfig example <account_id>/<tunnel_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct TunnelConfigArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration block for Tunnel Configuration.
    #[builder(into)]
    pub config: pulumi_wasm_rust::Output<crate::types::TunnelConfigConfig>,
    /// Identifier of the Tunnel to target for this configuration.
    #[builder(into)]
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
}

pub struct TunnelConfigResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration block for Tunnel Configuration.
    pub config: pulumi_wasm_rust::Output<crate::types::TunnelConfigConfig>,
    /// Identifier of the Tunnel to target for this configuration.
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TunnelConfigArgs) -> TunnelConfigResult {

    let result = crate::bindings::pulumi::cloudflare::tunnel_config::invoke(name, &crate::bindings::pulumi::cloudflare::tunnel_config::Args {
        account_id: &args.account_id.get_inner(),
        config: &args.config.get_inner(),
        tunnel_id: &args.tunnel_id.get_inner(),
    });

    TunnelConfigResult {
        account_id: crate::into_domain(result.account_id),
        config: crate::into_domain(result.config),
        tunnel_id: crate::into_domain(result.tunnel_id),
    }
}

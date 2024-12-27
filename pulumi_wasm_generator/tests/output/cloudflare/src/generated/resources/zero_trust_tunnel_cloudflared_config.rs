/// Provides a Cloudflare Tunnel configuration resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleTunnel:
///     type: cloudflare:ZeroTrustTunnelCloudflared
///     name: example_tunnel
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       name: example_tunnel
///       secret: <32 character secret>
///   exampleConfig:
///     type: cloudflare:ZeroTrustTunnelCloudflaredConfig
///     name: example_config
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       tunnelId: ${exampleTunnel.id}
///       config:
///         warpRouting:
///           enabled: true
///         originRequest:
///           connectTimeout: 1m0s
///           tlsTimeout: 1m0s
///           tcpKeepAlive: 1m0s
///           noHappyEyeballs: false
///           keepAliveConnections: 1024
///           keepAliveTimeout: 1m0s
///           httpHostHeader: baz
///           originServerName: foobar
///           caPool: /path/to/unsigned/ca/pool
///           noTlsVerify: false
///           disableChunkedEncoding: false
///           bastionMode: false
///           proxyAddress: 10.0.0.1
///           proxyPort: '8123'
///           proxyType: socks
///           ipRules:
///             - prefix: /web
///               ports:
///                 - 80
///                 - 443
///               allow: false
///         ingressRules:
///           - hostname: foo
///             path: /bar
///             service: http://10.0.0.2:8080
///             originRequest:
///               connectTimeout: 2m0s
///               access:
///                 required: true
///                 teamName: terraform
///                 audTags:
///                   - AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
///           - service: https://10.0.0.3:8081
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustTunnelCloudflaredConfig:ZeroTrustTunnelCloudflaredConfig example <account_id>/<tunnel_id>
/// ```
///
pub mod zero_trust_tunnel_cloudflared_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustTunnelCloudflaredConfigArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Configuration block for Tunnel Configuration.
        #[builder(into)]
        pub config: pulumi_wasm_rust::Output<
            super::types::ZeroTrustTunnelCloudflaredConfigConfig,
        >,
        /// Identifier of the Tunnel to target for this configuration.
        #[builder(into)]
        pub tunnel_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustTunnelCloudflaredConfigResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Configuration block for Tunnel Configuration.
        pub config: pulumi_wasm_rust::Output<
            super::types::ZeroTrustTunnelCloudflaredConfigConfig,
        >,
        /// Identifier of the Tunnel to target for this configuration.
        pub tunnel_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ZeroTrustTunnelCloudflaredConfigArgs,
    ) -> ZeroTrustTunnelCloudflaredConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let config_binding = args.config.get_inner();
        let tunnel_id_binding = args.tunnel_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustTunnelCloudflaredConfig:ZeroTrustTunnelCloudflaredConfig"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
                },
                register_interface::ObjectField {
                    name: "tunnelId".into(),
                    value: &tunnel_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "config".into(),
                },
                register_interface::ResultField {
                    name: "tunnelId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ZeroTrustTunnelCloudflaredConfigResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("config").unwrap(),
            ),
            tunnel_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tunnelId").unwrap(),
            ),
        }
    }
}

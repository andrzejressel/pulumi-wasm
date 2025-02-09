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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_tunnel_cloudflared_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustTunnelCloudflaredConfigArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for Tunnel Configuration.
        #[builder(into)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            super::types::ZeroTrustTunnelCloudflaredConfigConfig,
        >,
        /// Identifier of the Tunnel to target for this configuration.
        #[builder(into)]
        pub tunnel_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustTunnelCloudflaredConfigResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for Tunnel Configuration.
        pub config: pulumi_gestalt_rust::Output<
            super::types::ZeroTrustTunnelCloudflaredConfigConfig,
        >,
        /// Identifier of the Tunnel to target for this configuration.
        pub tunnel_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZeroTrustTunnelCloudflaredConfigArgs,
    ) -> ZeroTrustTunnelCloudflaredConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let config_binding_1 = args.config.get_output(context);
        let config_binding = config_binding_1.get_inner();
        let tunnel_id_binding_1 = args.tunnel_id.get_output(context);
        let tunnel_id_binding = tunnel_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustTunnelCloudflaredConfig:ZeroTrustTunnelCloudflaredConfig"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustTunnelCloudflaredConfigResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("config"),
            ),
            tunnel_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tunnelId"),
            ),
        }
    }
}

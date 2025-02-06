/// Tunnel exposes applications running on your local web server on any
/// network with an internet connection without manually adding DNS
/// records or configuring a firewall or router.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zero_trust_tunnel_cloudflared::create(
///         "example",
///         ZeroTrustTunnelCloudflaredArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .name("my-tunnel")
///             .secret("AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustTunnelCloudflared:ZeroTrustTunnelCloudflared example <account_id>/<tunnel_id>
/// ```
///
pub mod zero_trust_tunnel_cloudflared {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustTunnelCloudflaredArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Indicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard or using tunnel*config, tunnel*route or tunnel*virtual*network resources. Available values: `local`, `cloudflare`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub config_src: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A user-friendly name chosen when the tunnel is created. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// 32 or more bytes, encoded as a base64 string. The Create Argo Tunnel endpoint sets this as the tunnel's password. Anyone wishing to run the tunnel needs this password. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub secret: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZeroTrustTunnelCloudflaredArgs,
    ) -> ZeroTrustTunnelCloudflaredResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let config_src_binding = args.config_src.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let secret_binding = args.secret.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustTunnelCloudflared:ZeroTrustTunnelCloudflared"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "configSrc".into(),
                    value: &config_src_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "secret".into(),
                    value: &secret_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustTunnelCloudflaredResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            cname: pulumi_wasm_rust::__private::into_domain(o.extract_field("cname")),
            config_src: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configSrc"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            secret: pulumi_wasm_rust::__private::into_domain(o.extract_field("secret")),
            tunnel_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tunnelToken"),
            ),
        }
    }
}

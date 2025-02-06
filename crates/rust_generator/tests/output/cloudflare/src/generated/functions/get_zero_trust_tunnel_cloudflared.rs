pub mod get_zero_trust_tunnel_cloudflared {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZeroTrustTunnelCloudflaredArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// If true, only include deleted tunnels. If false, exclude deleted tunnels. If empty, all tunnels will be included. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub is_deleted: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Name of the tunnel. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetZeroTrustTunnelCloudflaredResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// ID of the tunnel.
        pub id: pulumi_wasm_rust::Output<String>,
        /// If true, only include deleted tunnels. If false, exclude deleted tunnels. If empty, all tunnels will be included. **Modifying this attribute will force creation of a new resource.**
        pub is_deleted: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the tunnel. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether the tunnel can be configured remotely from the Zero Trust dashboard.
        pub remote_config: pulumi_wasm_rust::Output<bool>,
        /// The status of the tunnel. Available values: `inactive`, `degraded`, `healthy`, `down`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The type of the tunnel. Available values: `cfd_tunnel`, `warp_connector`.
        pub tunnel_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetZeroTrustTunnelCloudflaredArgs,
    ) -> GetZeroTrustTunnelCloudflaredResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let is_deleted_binding = args.is_deleted.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getZeroTrustTunnelCloudflared:getZeroTrustTunnelCloudflared"
                .into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "isDeleted".into(),
                    value: &is_deleted_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetZeroTrustTunnelCloudflaredResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            is_deleted: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isDeleted"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            remote_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("remoteConfig"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tunnel_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tunnelType"),
            ),
        }
    }
}

/// Provides a Cloudflare Split Tunnel resource. Split tunnels are used to either
/// include or exclude lists of routes from the WARP client's tunnel.
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_split_tunnel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustSplitTunnelArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
        #[builder(into)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The settings policy for which to configure this split tunnel policy.
        #[builder(into, default)]
        pub policy_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The value of the tunnel attributes.
        #[builder(into)]
        pub tunnels: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::types::ZeroTrustSplitTunnelTunnel>,
        >,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustSplitTunnelResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
        pub mode: pulumi_gestalt_rust::Output<String>,
        /// The settings policy for which to configure this split tunnel policy.
        pub policy_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The value of the tunnel attributes.
        pub tunnels: pulumi_gestalt_rust::Output<
            Vec<super::types::ZeroTrustSplitTunnelTunnel>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZeroTrustSplitTunnelArgs,
    ) -> ZeroTrustSplitTunnelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let mode_binding_1 = args.mode.get_output(context);
        let mode_binding = mode_binding_1.get_inner();
        let policy_id_binding_1 = args.policy_id.get_output(context);
        let policy_id_binding = policy_id_binding_1.get_inner();
        let tunnels_binding_1 = args.tunnels.get_output(context);
        let tunnels_binding = tunnels_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustSplitTunnel:ZeroTrustSplitTunnel".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding,
                },
                register_interface::ObjectField {
                    name: "tunnels".into(),
                    value: &tunnels_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustSplitTunnelResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            mode: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mode")),
            policy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyId"),
            ),
            tunnels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tunnels"),
            ),
        }
    }
}

/// Provides a resource, that manages Cloudflare tunnel virtual networks
/// for Zero Trust. Tunnel virtual networks are used for segregation of
/// Tunnel IP Routes via Virtualized Networks to handle overlapping
/// private IPs in your origins.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zero_trust_tunnel_virtual_network::create(
///         "example",
///         ZeroTrustTunnelVirtualNetworkArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .comment("New tunnel virtual network for documentation")
///             .name("vnet-for-documentation")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustTunnelVirtualNetwork:ZeroTrustTunnelVirtualNetwork example <account_id>/<vnet_id>
/// ```
///
pub mod zero_trust_tunnel_virtual_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustTunnelVirtualNetworkArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Description of the tunnel virtual network.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
        #[builder(into, default)]
        pub is_default_network: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A user-friendly name chosen when the virtual network is created.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustTunnelVirtualNetworkResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Description of the tunnel virtual network.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
        pub is_default_network: pulumi_wasm_rust::Output<Option<bool>>,
        /// A user-friendly name chosen when the virtual network is created.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZeroTrustTunnelVirtualNetworkArgs,
    ) -> ZeroTrustTunnelVirtualNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let comment_binding = args.comment.get_output(context).get_inner();
        let is_default_network_binding = args
            .is_default_network
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustTunnelVirtualNetwork:ZeroTrustTunnelVirtualNetwork"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "isDefaultNetwork".into(),
                    value: &is_default_network_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustTunnelVirtualNetworkResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            is_default_network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isDefaultNetwork"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}

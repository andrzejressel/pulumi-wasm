pub mod get_tunnel_virtual_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTunnelVirtualNetworkArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Virtual Network Name.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTunnelVirtualNetworkResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The Virtual Network Comment.
        pub comment: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// If true, only include deleted virtual networks. If false, exclude deleted virtual networks. If empty, all virtual networks will be included.
        pub is_default: pulumi_wasm_rust::Output<bool>,
        /// The Virtual Network Name.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTunnelVirtualNetworkArgs,
    ) -> GetTunnelVirtualNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getTunnelVirtualNetwork:getTunnelVirtualNetwork"
                .into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTunnelVirtualNetworkResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            is_default: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isDefault"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}

pub mod get_virtual_wan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualWanArgs {
        /// The name of this Virtual Wan.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Virtual Wan exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualWanResult {
        /// Is branch to branch traffic is allowed?
        pub allow_branch_to_branch_traffic: pulumi_wasm_rust::Output<bool>,
        /// Is VPN Encryption disabled?
        pub disable_vpn_encryption: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Virtual Wan exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Office365 Local Breakout Category.
        pub office365_local_breakout_category: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Type of Virtual Wan (Basic or Standard).
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Virtual Wan.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Virtual Hubs IDs attached to this Virtual WAN.
        pub virtual_hub_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of VPN Site IDs attached to this Virtual WAN.
        pub vpn_site_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVirtualWanArgs,
    ) -> GetVirtualWanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVirtualWan:getVirtualWan".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVirtualWanResult {
            allow_branch_to_branch_traffic: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowBranchToBranchTraffic"),
            ),
            disable_vpn_encryption: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disableVpnEncryption"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            office365_local_breakout_category: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("office365LocalBreakoutCategory"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            virtual_hub_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualHubIds"),
            ),
            vpn_site_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpnSiteIds"),
            ),
        }
    }
}

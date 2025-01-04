pub mod get_virtual_wan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualWanArgs {
        /// The name of this Virtual Wan.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Virtual Wan exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetVirtualWanArgs) -> GetVirtualWanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVirtualWan:getVirtualWan".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowBranchToBranchTraffic".into(),
                },
                register_interface::ResultField {
                    name: "disableVpnEncryption".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "office365LocalBreakoutCategory".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualHubIds".into(),
                },
                register_interface::ResultField {
                    name: "vpnSiteIds".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVirtualWanResult {
            allow_branch_to_branch_traffic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowBranchToBranchTraffic").unwrap(),
            ),
            disable_vpn_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableVpnEncryption").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            office365_local_breakout_category: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("office365LocalBreakoutCategory").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_hub_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualHubIds").unwrap(),
            ),
            vpn_site_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnSiteIds").unwrap(),
            ),
        }
    }
}

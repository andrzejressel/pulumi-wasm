pub mod get_virtual_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualNetworkArgs {
        /// Specifies the name of the Dev Test Lab.
        #[builder(into)]
        pub lab_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Virtual Network.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group that contains the Virtual Network.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetVirtualNetworkResult {
        /// The list of subnets enabled for the virtual network as defined below.
        pub allowed_subnets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::devtest::GetVirtualNetworkAllowedSubnet>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub lab_name: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The list of permission overrides for the subnets as defined below.
        pub subnet_overrides: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::devtest::GetVirtualNetworkSubnetOverride>,
        >,
        /// The unique immutable identifier of the virtual network.
        pub unique_identifier: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVirtualNetworkArgs) -> GetVirtualNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let lab_name_binding = args.lab_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:devtest/getVirtualNetwork:getVirtualNetwork".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "labName".into(),
                    value: &lab_name_binding,
                },
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
                    name: "allowedSubnets".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "labName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "subnetOverrides".into(),
                },
                register_interface::ResultField {
                    name: "uniqueIdentifier".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVirtualNetworkResult {
            allowed_subnets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowedSubnets").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            lab_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            subnet_overrides: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetOverrides").unwrap(),
            ),
            unique_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueIdentifier").unwrap(),
            ),
        }
    }
}

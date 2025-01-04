pub mod get_subnet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubnetArgs {
        /// Name of the resource.
        /// UserDefined subnets are named in the format of "service-n", where n ranges from 1 to 5.
        /// Management subnets have arbitary names including "vmotion", "vsan", "system-management" etc. More details about subnet names can be found on the cloud console.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource name of the private cloud that this subnet belongs.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSubnetResult {
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub dhcp_address_ranges: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetSubnetDhcpAddressRange>,
        >,
        pub gateway_id: pulumi_wasm_rust::Output<String>,
        pub gateway_ip: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ip_cidr_range: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub parent: pulumi_wasm_rust::Output<String>,
        pub standard_config: pulumi_wasm_rust::Output<bool>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub uid: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
        pub vlan_id: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSubnetArgs) -> GetSubnetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let parent_binding = args.parent.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getSubnet:getSubnet".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dhcpAddressRanges".into(),
                },
                register_interface::ResultField {
                    name: "gatewayId".into(),
                },
                register_interface::ResultField {
                    name: "gatewayIp".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipCidrRange".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "standardConfig".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "vlanId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSubnetResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            dhcp_address_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dhcpAddressRanges").unwrap(),
            ),
            gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayId").unwrap(),
            ),
            gateway_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayIp").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_cidr_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipCidrRange").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            standard_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("standardConfig").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            vlan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vlanId").unwrap(),
            ),
        }
    }
}

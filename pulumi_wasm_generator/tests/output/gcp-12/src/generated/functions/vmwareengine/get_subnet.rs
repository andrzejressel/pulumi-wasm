pub mod get_subnet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubnetArgs {
        /// Name of the resource.
        /// UserDefined subnets are named in the format of "service-n", where n ranges from 1 to 5.
        /// Management subnets have arbitary names including "vmotion", "vsan", "system-management" etc. More details about subnet names can be found on the cloud console.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource name of the private cloud that this subnet belongs.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSubnetArgs,
    ) -> GetSubnetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getSubnet:getSubnet".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSubnetResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            dhcp_address_ranges: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dhcpAddressRanges"),
            ),
            gateway_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gatewayId"),
            ),
            gateway_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gatewayIp"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ip_cidr_range: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipCidrRange"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            standard_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("standardConfig"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            vlan_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vlanId")),
        }
    }
}

pub mod get_nat_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNatGatewayArgs {
        /// Custom filter block as described below.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetNatGatewayFilter>>,
        >,
        /// ID of the specific NAT Gateway to retrieve.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// State of the NAT Gateway (pending | failed | available | deleting | deleted ).
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of subnet that the NAT Gateway resides in.
        #[builder(into, default)]
        pub subnet_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match
        /// a pair on the desired NAT Gateway.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the VPC that the NAT Gateway resides in.
        #[builder(into, default)]
        pub vpc_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNatGatewayResult {
        /// ID of the EIP allocated to the selected NAT Gateway.
        pub allocation_id: pulumi_wasm_rust::Output<String>,
        /// The association ID of the Elastic IP address that's associated with the NAT Gateway. Only available when `connectivity_type` is `public`.
        pub association_id: pulumi_wasm_rust::Output<String>,
        /// Connectivity type of the NAT Gateway.
        pub connectivity_type: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetNatGatewayFilter>>,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        /// The ID of the ENI allocated to the selected NAT Gateway.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// Private IP address of the selected NAT Gateway.
        pub private_ip: pulumi_wasm_rust::Output<String>,
        /// Public IP (EIP) address of the selected NAT Gateway.
        pub public_ip: pulumi_wasm_rust::Output<String>,
        /// Secondary allocation EIP IDs for the selected NAT Gateway.
        pub secondary_allocation_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The number of secondary private IPv4 addresses assigned to the selected NAT Gateway.
        pub secondary_private_ip_address_count: pulumi_wasm_rust::Output<i32>,
        /// Secondary private IPv4 addresses assigned to the selected NAT Gateway.
        pub secondary_private_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNatGatewayArgs,
    ) -> GetNatGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getNatGateway:getNatGateway".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNatGatewayResult {
            allocation_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allocationId"),
            ),
            association_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("associationId"),
            ),
            connectivity_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectivityType"),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkInterfaceId"),
            ),
            private_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateIp"),
            ),
            public_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicIp"),
            ),
            secondary_allocation_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryAllocationIds"),
            ),
            secondary_private_ip_address_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryPrivateIpAddressCount"),
            ),
            secondary_private_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryPrivateIpAddresses"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            vpc_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}

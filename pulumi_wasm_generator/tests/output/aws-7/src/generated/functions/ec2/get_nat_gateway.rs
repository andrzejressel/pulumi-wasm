pub mod get_nat_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNatGatewayArgs {
        /// Custom filter block as described below.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetNatGatewayFilter>>,
        >,
        /// ID of the specific NAT Gateway to retrieve.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// State of the NAT Gateway (pending | failed | available | deleting | deleted ).
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of subnet that the NAT Gateway resides in.
        #[builder(into, default)]
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags, each pair of which must exactly match
        /// a pair on the desired NAT Gateway.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the VPC that the NAT Gateway resides in.
        #[builder(into, default)]
        pub vpc_id: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn invoke(args: GetNatGatewayArgs) -> GetNatGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let id_binding = args.id.get_inner();
        let state_binding = args.state.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getNatGateway:getNatGateway".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "allocationId".into(),
                },
                register_interface::ResultField {
                    name: "associationId".into(),
                },
                register_interface::ResultField {
                    name: "connectivityType".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "privateIp".into(),
                },
                register_interface::ResultField {
                    name: "publicIp".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAllocationIds".into(),
                },
                register_interface::ResultField {
                    name: "secondaryPrivateIpAddressCount".into(),
                },
                register_interface::ResultField {
                    name: "secondaryPrivateIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNatGatewayResult {
            allocation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationId").unwrap(),
            ),
            association_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationId").unwrap(),
            ),
            connectivity_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectivityType").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceId").unwrap(),
            ),
            private_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIp").unwrap(),
            ),
            public_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIp").unwrap(),
            ),
            secondary_allocation_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAllocationIds").unwrap(),
            ),
            secondary_private_ip_address_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryPrivateIpAddressCount").unwrap(),
            ),
            secondary_private_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryPrivateIpAddresses").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}

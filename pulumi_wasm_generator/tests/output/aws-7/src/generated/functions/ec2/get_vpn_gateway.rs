pub mod get_vpn_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpnGatewayArgs {
        /// Autonomous System Number (ASN) for the Amazon side of the specific VPN Gateway to retrieve.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub amazon_side_asn: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of a VPC attached to the specific VPN Gateway to retrieve.
        #[builder(into, default)]
        pub attached_vpc_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Availability Zone of the specific VPN Gateway to retrieve.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpnGatewayFilter>>,
        >,
        /// ID of the specific VPN Gateway to retrieve.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// State of the specific VPN Gateway to retrieve.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags, each pair of which must exactly match
        /// a pair on the desired VPN Gateway.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpnGatewayResult {
        pub amazon_side_asn: pulumi_wasm_rust::Output<String>,
        pub arn: pulumi_wasm_rust::Output<String>,
        pub attached_vpc_id: pulumi_wasm_rust::Output<String>,
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpnGatewayFilter>>,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVpnGatewayArgs) -> GetVpnGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let amazon_side_asn_binding = args.amazon_side_asn.get_inner();
        let attached_vpc_id_binding = args.attached_vpc_id.get_inner();
        let availability_zone_binding = args.availability_zone.get_inner();
        let filters_binding = args.filters.get_inner();
        let id_binding = args.id.get_inner();
        let state_binding = args.state.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getVpnGateway:getVpnGateway".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "amazonSideAsn".into(),
                    value: &amazon_side_asn_binding,
                },
                register_interface::ObjectField {
                    name: "attachedVpcId".into(),
                    value: &attached_vpc_id_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "amazonSideAsn".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "attachedVpcId".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVpnGatewayResult {
            amazon_side_asn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("amazonSideAsn").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            attached_vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachedVpcId").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}

pub mod get_local_gateway_route_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalGatewayRouteTableArgs {
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetLocalGatewayRouteTableFilter>>,
        >,
        /// ID of the specific local gateway route table to retrieve.
        #[builder(into, default)]
        pub local_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Local Gateway Route Table Id assigned to desired local gateway route table
        #[builder(into, default)]
        pub local_gateway_route_table_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the Outpost the local gateway route table is associated with.
        #[builder(into, default)]
        pub outpost_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// State of the local gateway route table.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match
        /// a pair on the desired local gateway route table.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLocalGatewayRouteTableResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetLocalGatewayRouteTableFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub local_gateway_id: pulumi_wasm_rust::Output<String>,
        pub local_gateway_route_table_id: pulumi_wasm_rust::Output<String>,
        pub outpost_arn: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetLocalGatewayRouteTableArgs,
    ) -> GetLocalGatewayRouteTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let local_gateway_id_binding = args.local_gateway_id.get_inner();
        let local_gateway_route_table_id_binding = args
            .local_gateway_route_table_id
            .get_inner();
        let outpost_arn_binding = args.outpost_arn.get_inner();
        let state_binding = args.state.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getLocalGatewayRouteTable:getLocalGatewayRouteTable".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "localGatewayId".into(),
                    value: &local_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "localGatewayRouteTableId".into(),
                    value: &local_gateway_route_table_id_binding,
                },
                register_interface::ObjectField {
                    name: "outpostArn".into(),
                    value: &outpost_arn_binding,
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
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "localGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "localGatewayRouteTableId".into(),
                },
                register_interface::ResultField {
                    name: "outpostArn".into(),
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
        GetLocalGatewayRouteTableResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            local_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localGatewayId").unwrap(),
            ),
            local_gateway_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localGatewayRouteTableId").unwrap(),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostArn").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
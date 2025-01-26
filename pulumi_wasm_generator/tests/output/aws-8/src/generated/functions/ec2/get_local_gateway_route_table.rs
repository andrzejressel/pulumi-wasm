pub mod get_local_gateway_route_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalGatewayRouteTableArgs {
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetLocalGatewayRouteTableFilter>>,
        >,
        /// ID of the specific local gateway route table to retrieve.
        #[builder(into, default)]
        pub local_gateway_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Local Gateway Route Table Id assigned to desired local gateway route table
        #[builder(into, default)]
        pub local_gateway_route_table_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// ARN of the Outpost the local gateway route table is associated with.
        #[builder(into, default)]
        pub outpost_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// State of the local gateway route table.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match
        /// a pair on the desired local gateway route table.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLocalGatewayRouteTableArgs,
    ) -> GetLocalGatewayRouteTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let local_gateway_id_binding = args
            .local_gateway_id
            .get_output(context)
            .get_inner();
        let local_gateway_route_table_id_binding = args
            .local_gateway_route_table_id
            .get_output(context)
            .get_inner();
        let outpost_arn_binding = args.outpost_arn.get_output(context).get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getLocalGatewayRouteTable:getLocalGatewayRouteTable".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLocalGatewayRouteTableResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            local_gateway_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("localGatewayId"),
            ),
            local_gateway_route_table_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("localGatewayRouteTableId"),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outpostArn"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}

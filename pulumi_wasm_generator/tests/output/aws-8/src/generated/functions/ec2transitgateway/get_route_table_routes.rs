pub mod get_route_table_routes {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouteTableRoutesArgs {
        /// Custom filter block as described below.
        #[builder(into)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::super::types::ec2transitgateway::GetRouteTableRoutesFilter>,
        >,
        /// Identifier of EC2 Transit Gateway Route Table.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRouteTableRoutesResult {
        pub filters: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2transitgateway::GetRouteTableRoutesFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of Transit Gateway Routes.
        pub routes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2transitgateway::GetRouteTableRoutesRoute>,
        >,
        pub transit_gateway_route_table_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRouteTableRoutesArgs,
    ) -> GetRouteTableRoutesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2transitgateway/getRouteTableRoutes:getRouteTableRoutes"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayRouteTableId".into(),
                    value: &transit_gateway_route_table_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRouteTableRoutesResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            routes: pulumi_wasm_rust::__private::into_domain(o.extract_field("routes")),
            transit_gateway_route_table_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("transitGatewayRouteTableId"),
            ),
        }
    }
}

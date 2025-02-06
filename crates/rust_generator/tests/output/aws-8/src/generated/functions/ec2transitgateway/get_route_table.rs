pub mod get_route_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouteTableArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetRouteTableFilter>,
            >,
        >,
        /// Identifier of the EC2 Transit Gateway Route Table.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the EC2 Transit Gateway Route Table
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRouteTableResult {
        /// EC2 Transit Gateway Route Table ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Boolean whether this is the default association route table for the EC2 Transit Gateway
        pub default_association_route_table: pulumi_wasm_rust::Output<bool>,
        /// Boolean whether this is the default propagation route table for the EC2 Transit Gateway
        pub default_propagation_route_table: pulumi_wasm_rust::Output<bool>,
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetRouteTableFilter>,
            >,
        >,
        /// EC2 Transit Gateway Route Table identifier
        pub id: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Route Table
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// EC2 Transit Gateway identifier
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRouteTableArgs,
    ) -> GetRouteTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2transitgateway/getRouteTable:getRouteTable".into(),
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRouteTableResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            default_association_route_table: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultAssociationRouteTable"),
            ),
            default_propagation_route_table: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultPropagationRouteTable"),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("transitGatewayId"),
            ),
        }
    }
}

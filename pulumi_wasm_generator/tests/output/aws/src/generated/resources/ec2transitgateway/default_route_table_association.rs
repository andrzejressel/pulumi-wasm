/// Resource for managing an AWS EC2 (Elastic Compute Cloud) Transit Gateway Default Route Table Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = default_route_table_association::create(
///         "example",
///         DefaultRouteTableAssociationArgs::builder()
///             .transit_gateway_id("${exampleAwsEc2TransitGateway.id}")
///             .transit_gateway_route_table_id(
///                 "${exampleAwsEc2TransitGatewayRouteTable.id}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
pub mod default_route_table_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultRouteTableAssociationArgs {
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ec2transitgateway::DefaultRouteTableAssociationTimeouts,
            >,
        >,
        /// ID of the Transit Gateway to change the default association route table on.
        #[builder(into)]
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
        /// ID of the Transit Gateway Route Table to be made the default association route table.
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultRouteTableAssociationResult {
        pub original_default_route_table_id: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ec2transitgateway::DefaultRouteTableAssociationTimeouts,
            >,
        >,
        /// ID of the Transit Gateway to change the default association route table on.
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
        /// ID of the Transit Gateway Route Table to be made the default association route table.
        pub transit_gateway_route_table_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DefaultRouteTableAssociationArgs,
    ) -> DefaultRouteTableAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let timeouts_binding = args.timeouts.get_inner();
        let transit_gateway_id_binding = args.transit_gateway_id.get_inner();
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/defaultRouteTableAssociation:DefaultRouteTableAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayRouteTableId".into(),
                    value: &transit_gateway_route_table_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "originalDefaultRouteTableId".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayRouteTableId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DefaultRouteTableAssociationResult {
            original_default_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originalDefaultRouteTableId").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
            transit_gateway_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayRouteTableId").unwrap(),
            ),
        }
    }
}
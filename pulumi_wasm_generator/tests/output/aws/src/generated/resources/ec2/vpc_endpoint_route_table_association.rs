/// Manages a VPC Endpoint Route Table Association
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc_endpoint_route_table_association::create(
///         "example",
///         VpcEndpointRouteTableAssociationArgs::builder()
///             .route_table_id("${exampleAwsRouteTable.id}")
///             .vpc_endpoint_id("${exampleAwsVpcEndpoint.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Endpoint Route Table Associations using `vpc_endpoint_id` together with `route_table_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcEndpointRouteTableAssociation:VpcEndpointRouteTableAssociation example vpce-aaaaaaaa/rtb-bbbbbbbb
/// ```
pub mod vpc_endpoint_route_table_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointRouteTableAssociationArgs {
        /// Identifier of the EC2 Route Table to be associated with the VPC Endpoint.
        #[builder(into)]
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the VPC Endpoint with which the EC2 Route Table will be associated.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointRouteTableAssociationResult {
        /// Identifier of the EC2 Route Table to be associated with the VPC Endpoint.
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the VPC Endpoint with which the EC2 Route Table will be associated.
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VpcEndpointRouteTableAssociationArgs,
    ) -> VpcEndpointRouteTableAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let route_table_id_binding = args.route_table_id.get_inner();
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointRouteTableAssociation:VpcEndpointRouteTableAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "routeTableId".into(),
                    value: &route_table_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "routeTableId".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpointId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcEndpointRouteTableAssociationResult {
            route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeTableId").unwrap(),
            ),
            vpc_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointId").unwrap(),
            ),
        }
    }
}
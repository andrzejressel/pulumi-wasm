/// Provides a resource to create an association between a route table and a subnet or a route table and an
/// internet gateway or virtual private gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let a = route_table_association::create(
///         "a",
///         RouteTableAssociationArgs::builder()
///             .route_table_id("${bar.id}")
///             .subnet_id("${foo.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let b = route_table_association::create(
///         "b",
///         RouteTableAssociationArgs::builder()
///             .gateway_id("${foo.id}")
///             .route_table_id("${bar.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// With EC2 Internet Gateways:
///
/// __Using `pulumi import` to import__ EC2 Route Table Associations using the associated resource ID and Route Table ID separated by a forward slash (`/`). For example:
///
/// With EC2 Subnets:
///
/// ```sh
/// $ pulumi import aws:ec2/routeTableAssociation:RouteTableAssociation assoc subnet-6777656e646f6c796e/rtb-656c65616e6f72
/// ```
/// With EC2 Internet Gateways:
///
/// ```sh
/// $ pulumi import aws:ec2/routeTableAssociation:RouteTableAssociation assoc igw-01b3a60780f8d034a/rtb-656c65616e6f72
/// ```
pub mod route_table_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteTableAssociationArgs {
        /// The gateway ID to create an association. Conflicts with `subnet_id`.
        #[builder(into, default)]
        pub gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the routing table to associate with.
        #[builder(into)]
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// The subnet ID to create an association. Conflicts with `gateway_id`.
        #[builder(into, default)]
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RouteTableAssociationResult {
        /// The gateway ID to create an association. Conflicts with `subnet_id`.
        pub gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the routing table to associate with.
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// The subnet ID to create an association. Conflicts with `gateway_id`.
        pub subnet_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RouteTableAssociationArgs,
    ) -> RouteTableAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let gateway_id_binding = args.gateway_id.get_inner();
        let route_table_id_binding = args.route_table_id.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/routeTableAssociation:RouteTableAssociation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "gatewayId".into(),
                    value: &gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "routeTableId".into(),
                    value: &route_table_id_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "gatewayId".into(),
                },
                register_interface::ResultField {
                    name: "routeTableId".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RouteTableAssociationResult {
            gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayId").unwrap(),
            ),
            route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeTableId").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
        }
    }
}

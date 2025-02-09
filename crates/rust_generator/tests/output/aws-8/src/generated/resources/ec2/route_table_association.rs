/// Provides a resource to create an association between a route table and a subnet or a route table and an
/// internet gateway or virtual private gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route_table_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteTableAssociationArgs {
        /// The gateway ID to create an association. Conflicts with `subnet_id`.
        #[builder(into, default)]
        pub gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the routing table to associate with.
        #[builder(into)]
        pub route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The subnet ID to create an association. Conflicts with `gateway_id`.
        #[builder(into, default)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RouteTableAssociationResult {
        /// The gateway ID to create an association. Conflicts with `subnet_id`.
        pub gateway_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the routing table to associate with.
        pub route_table_id: pulumi_gestalt_rust::Output<String>,
        /// The subnet ID to create an association. Conflicts with `gateway_id`.
        pub subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteTableAssociationArgs,
    ) -> RouteTableAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let gateway_id_binding = args.gateway_id.get_output(context);
        let route_table_id_binding = args.route_table_id.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/routeTableAssociation:RouteTableAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayId".into(),
                    value: gateway_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeTableId".into(),
                    value: route_table_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouteTableAssociationResult {
            gateway_id: o.get_field("gatewayId"),
            route_table_id: o.get_field("routeTableId"),
            subnet_id: o.get_field("subnetId"),
        }
    }
}

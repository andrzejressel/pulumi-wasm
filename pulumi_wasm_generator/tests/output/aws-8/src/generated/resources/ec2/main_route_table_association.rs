/// Provides a resource for managing the main routing table of a VPC.
///
/// > **NOTE:** **Do not** use both `aws.ec2.DefaultRouteTable` to manage a default route table **and** `aws.ec2.MainRouteTableAssociation` with the same VPC due to possible route conflicts. See aws.ec2.DefaultRouteTable documentation for more details.
/// For more information, see the Amazon VPC User Guide on [Route Tables][aws-route-tables]. For information about managing normal route tables in Pulumi, see [`aws.ec2.RouteTable`][tf-route-tables].
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let a = main_route_table_association::create(
///         "a",
///         MainRouteTableAssociationArgs::builder()
///             .route_table_id("${bar.id}")
///             .vpc_id("${foo.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Notes
///
/// On VPC creation, the AWS API always creates an initial Main Route Table. This
/// resource records the ID of that Route Table under `original_route_table_id`.
/// The "Delete" action for a `main_route_table_association` consists of resetting
/// this original table as the Main Route Table for the VPC. You'll see this
/// additional Route Table in the AWS console; it must remain intact in order for
/// the `main_route_table_association` delete to work properly.
pub mod main_route_table_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MainRouteTableAssociationArgs {
        /// The ID of the Route Table to set as the new
        /// main route table for the target VPC
        #[builder(into)]
        pub route_table_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the VPC whose main route table should be set
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MainRouteTableAssociationResult {
        /// Used internally, see **Notes** below
        pub original_route_table_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Route Table to set as the new
        /// main route table for the target VPC
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPC whose main route table should be set
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MainRouteTableAssociationArgs,
    ) -> MainRouteTableAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let route_table_id_binding = args.route_table_id.get_output(context).get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/mainRouteTableAssociation:MainRouteTableAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "routeTableId".into(),
                    value: &route_table_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "originalRouteTableId".into(),
                },
                register_interface::ResultField {
                    name: "routeTableId".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MainRouteTableAssociationResult {
            original_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originalRouteTableId").unwrap(),
            ),
            route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeTableId").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}

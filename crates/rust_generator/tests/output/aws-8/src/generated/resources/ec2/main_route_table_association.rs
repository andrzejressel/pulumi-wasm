/// Provides a resource for managing the main routing table of a VPC.
///
/// > **NOTE:** **Do not** use both `aws.ec2.DefaultRouteTable` to manage a default route table **and** `aws.ec2.MainRouteTableAssociation` with the same VPC due to possible route conflicts. See aws.ec2.DefaultRouteTable documentation for more details.
/// For more information, see the Amazon VPC User Guide on [Route Tables][aws-route-tables]. For information about managing normal route tables in Pulumi, see [`aws.ec2.RouteTable`][tf-route-tables].
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod main_route_table_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MainRouteTableAssociationArgs {
        /// The ID of the Route Table to set as the new
        /// main route table for the target VPC
        #[builder(into)]
        pub route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPC whose main route table should be set
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MainRouteTableAssociationResult {
        /// Used internally, see **Notes** below
        pub original_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Route Table to set as the new
        /// main route table for the target VPC
        pub route_table_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC whose main route table should be set
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MainRouteTableAssociationArgs,
    ) -> MainRouteTableAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let route_table_id_binding = args.route_table_id.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/mainRouteTableAssociation:MainRouteTableAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeTableId".into(),
                    value: &route_table_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MainRouteTableAssociationResult {
            original_route_table_id: o.get_field("originalRouteTableId"),
            route_table_id: o.get_field("routeTableId"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}

/// Manages an EC2 Transit Gateway Route Table propagation.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = route_table_propagation::create(
///         "example",
///         RouteTablePropagationArgs::builder()
///             .transit_gateway_attachment_id(
///                 "${exampleAwsEc2TransitGatewayVpcAttachment.id}",
///             )
///             .transit_gateway_route_table_id(
///                 "${exampleAwsEc2TransitGatewayRouteTable.id}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_route_table_propagation` using the EC2 Transit Gateway Route Table identifier, an underscore, and the EC2 Transit Gateway Attachment identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/routeTablePropagation:RouteTablePropagation example tgw-rtb-12345678_tgw-attach-87654321
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route_table_propagation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteTablePropagationArgs {
        /// Identifier of EC2 Transit Gateway Attachment.
        #[builder(into)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of EC2 Transit Gateway Route Table.
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteTablePropagationResult {
        /// Identifier of the resource
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// Type of the resource
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Attachment.
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Route Table.
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteTablePropagationArgs,
    ) -> RouteTablePropagationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(context);
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/routeTablePropagation:RouteTablePropagation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: transit_gateway_attachment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayRouteTableId".into(),
                    value: transit_gateway_route_table_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouteTablePropagationResult {
            resource_id: o.get_field("resourceId"),
            resource_type: o.get_field("resourceType"),
            transit_gateway_attachment_id: o.get_field("transitGatewayAttachmentId"),
            transit_gateway_route_table_id: o.get_field("transitGatewayRouteTableId"),
        }
    }
}

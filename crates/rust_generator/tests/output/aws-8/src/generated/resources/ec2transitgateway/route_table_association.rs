/// Manages an EC2 Transit Gateway Route Table association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = route_table_association::create(
///         "example",
///         RouteTableAssociationArgs::builder()
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
/// Using `pulumi import`, import `aws_ec2_transit_gateway_route_table_association` using the EC2 Transit Gateway Route Table identifier, an underscore, and the EC2 Transit Gateway Attachment identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/routeTableAssociation:RouteTableAssociation example tgw-rtb-12345678_tgw-attach-87654321
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route_table_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteTableAssociationArgs {
        /// Boolean whether the Gateway Attachment should remove any current Route Table association before associating with the specified Route Table. Default value: `false`. This argument is intended for use with EC2 Transit Gateways shared into the current account, otherwise the `transit_gateway_default_route_table_association` argument of the `aws.ec2transitgateway.VpcAttachment` resource should be used.
        #[builder(into, default)]
        pub replace_existing_association: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Identifier of EC2 Transit Gateway Attachment.
        #[builder(into)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of EC2 Transit Gateway Route Table.
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteTableAssociationResult {
        /// Boolean whether the Gateway Attachment should remove any current Route Table association before associating with the specified Route Table. Default value: `false`. This argument is intended for use with EC2 Transit Gateways shared into the current account, otherwise the `transit_gateway_default_route_table_association` argument of the `aws.ec2transitgateway.VpcAttachment` resource should be used.
        pub replace_existing_association: pulumi_gestalt_rust::Output<Option<bool>>,
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RouteTableAssociationArgs,
    ) -> RouteTableAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let replace_existing_association_binding = args
            .replace_existing_association
            .get_output(context)
            .get_inner();
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(context)
            .get_inner();
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/routeTableAssociation:RouteTableAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "replaceExistingAssociation".into(),
                    value: &replace_existing_association_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayRouteTableId".into(),
                    value: &transit_gateway_route_table_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RouteTableAssociationResult {
            replace_existing_association: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replaceExistingAssociation"),
            ),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            resource_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceType"),
            ),
            transit_gateway_attachment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayAttachmentId"),
            ),
            transit_gateway_route_table_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayRouteTableId"),
            ),
        }
    }
}

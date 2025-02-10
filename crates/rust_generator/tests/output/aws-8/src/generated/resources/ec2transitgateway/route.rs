/// Manages an EC2 Transit Gateway Route.
///
/// ## Example Usage
///
/// ### Standard usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = route::create(
///         "example",
///         RouteArgs::builder()
///             .destination_cidr_block("0.0.0.0/0")
///             .transit_gateway_attachment_id(
///                 "${exampleAwsEc2TransitGatewayVpcAttachment.id}",
///             )
///             .transit_gateway_route_table_id(
///                 "${exampleAwsEc2TransitGateway.associationDefaultRouteTableId}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Blackhole route
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = route::create(
///         "example",
///         RouteArgs::builder()
///             .blackhole(true)
///             .destination_cidr_block("0.0.0.0/0")
///             .transit_gateway_route_table_id(
///                 "${exampleAwsEc2TransitGateway.associationDefaultRouteTableId}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_route` using the EC2 Transit Gateway Route Table, an underscore, and the destination. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/route:Route example tgw-rtb-12345678_0.0.0.0/0
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteArgs {
        /// Indicates whether to drop traffic that matches this route (default to `false`).
        #[builder(into, default)]
        pub blackhole: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// IPv4 or IPv6 RFC1924 CIDR used for destination matches. Routing decisions are based on the most specific match.
        #[builder(into)]
        pub destination_cidr_block: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of EC2 Transit Gateway Attachment (required if `blackhole` is set to false).
        #[builder(into, default)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Identifier of EC2 Transit Gateway Route Table.
        #[builder(into)]
        pub transit_gateway_route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteResult {
        /// Indicates whether to drop traffic that matches this route (default to `false`).
        pub blackhole: pulumi_gestalt_rust::Output<Option<bool>>,
        /// IPv4 or IPv6 RFC1924 CIDR used for destination matches. Routing decisions are based on the most specific match.
        pub destination_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Transit Gateway Attachment (required if `blackhole` is set to false).
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<Option<String>>,
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
        args: RouteArgs,
    ) -> RouteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let blackhole_binding = args.blackhole.get_output(context);
        let destination_cidr_block_binding = args
            .destination_cidr_block
            .get_output(context);
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(context);
        let transit_gateway_route_table_id_binding = args
            .transit_gateway_route_table_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/route:Route".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blackhole".into(),
                    value: blackhole_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationCidrBlock".into(),
                    value: destination_cidr_block_binding.get_id(),
                },
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
        RouteResult {
            blackhole: o.get_field("blackhole"),
            destination_cidr_block: o.get_field("destinationCidrBlock"),
            transit_gateway_attachment_id: o.get_field("transitGatewayAttachmentId"),
            transit_gateway_route_table_id: o.get_field("transitGatewayRouteTableId"),
        }
    }
}

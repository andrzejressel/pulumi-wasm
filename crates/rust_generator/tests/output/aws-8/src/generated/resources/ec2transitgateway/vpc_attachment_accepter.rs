/// Manages the accepter's side of an EC2 Transit Gateway VPC Attachment.
///
/// When a cross-account (requester's AWS account differs from the accepter's AWS account) EC2 Transit Gateway VPC Attachment
/// is created, an EC2 Transit Gateway VPC Attachment resource is automatically created in the accepter's account.
/// The requester can use the `aws.ec2transitgateway.VpcAttachment` resource to manage its side of the connection
/// and the accepter can use the `aws.ec2transitgateway.VpcAttachmentAccepter` resource to "adopt" its side of the
/// connection into management.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2transitgateway:VpcAttachmentAccepter
///     properties:
///       transitGatewayAttachmentId: ${exampleAwsEc2TransitGatewayVpcAttachment.id}
///       tags:
///         Name: Example cross-account attachment
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_vpc_attachment_accepter` using the EC2 Transit Gateway Attachment identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/vpcAttachmentAccepter:VpcAttachmentAccepter example tgw-attach-12345678
/// ```
pub mod vpc_attachment_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcAttachmentAccepterArgs {
        /// Key-value tags for the EC2 Transit Gateway VPC Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the EC2 Transit Gateway Attachment to manage.
        #[builder(into)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Boolean whether the VPC Attachment should be associated with the EC2 Transit Gateway association default route table. Default value: `true`.
        #[builder(into, default)]
        pub transit_gateway_default_route_table_association: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Boolean whether the VPC Attachment should propagate routes with the EC2 Transit Gateway propagation default route table. Default value: `true`.
        #[builder(into, default)]
        pub transit_gateway_default_route_table_propagation: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcAttachmentAccepterResult {
        /// Whether Appliance Mode support is enabled. Valid values: `disable`, `enable`.
        pub appliance_mode_support: pulumi_gestalt_rust::Output<String>,
        /// Whether DNS support is enabled. Valid values: `disable`, `enable`.
        pub dns_support: pulumi_gestalt_rust::Output<String>,
        /// Whether IPv6 support is enabled. Valid values: `disable`, `enable`.
        pub ipv6_support: pulumi_gestalt_rust::Output<String>,
        /// Whether Security Group Referencing Support is enabled. Valid values: `disable`, `enable`.
        pub security_group_referencing_support: pulumi_gestalt_rust::Output<String>,
        /// Identifiers of EC2 Subnets.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-value tags for the EC2 Transit Gateway VPC Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the EC2 Transit Gateway Attachment to manage.
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<String>,
        /// Boolean whether the VPC Attachment should be associated with the EC2 Transit Gateway association default route table. Default value: `true`.
        pub transit_gateway_default_route_table_association: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Boolean whether the VPC Attachment should propagate routes with the EC2 Transit Gateway propagation default route table. Default value: `true`.
        pub transit_gateway_default_route_table_propagation: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Identifier of EC2 Transit Gateway.
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 VPC.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the AWS account that owns the EC2 VPC.
        pub vpc_owner_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VpcAttachmentAccepterArgs,
    ) -> VpcAttachmentAccepterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_output(context).get_inner();
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(context)
            .get_inner();
        let transit_gateway_default_route_table_association_binding = args
            .transit_gateway_default_route_table_association
            .get_output(context)
            .get_inner();
        let transit_gateway_default_route_table_propagation_binding = args
            .transit_gateway_default_route_table_propagation
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/vpcAttachmentAccepter:VpcAttachmentAccepter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayDefaultRouteTableAssociation".into(),
                    value: &transit_gateway_default_route_table_association_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayDefaultRouteTablePropagation".into(),
                    value: &transit_gateway_default_route_table_propagation_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcAttachmentAccepterResult {
            appliance_mode_support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applianceModeSupport"),
            ),
            dns_support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsSupport"),
            ),
            ipv6_support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6Support"),
            ),
            security_group_referencing_support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupReferencingSupport"),
            ),
            subnet_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            transit_gateway_attachment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayAttachmentId"),
            ),
            transit_gateway_default_route_table_association: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayDefaultRouteTableAssociation"),
            ),
            transit_gateway_default_route_table_propagation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayDefaultRouteTablePropagation"),
            ),
            transit_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayId"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcId"),
            ),
            vpc_owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcOwnerId"),
            ),
        }
    }
}

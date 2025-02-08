/// Manages an EC2 Transit Gateway VPC Attachment. For examples of custom route table association and propagation, see the EC2 Transit Gateway Networking Examples Guide.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpc_attachment::create(
///         "example",
///         VpcAttachmentArgs::builder()
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .transit_gateway_id("${exampleAwsEc2TransitGateway.id}")
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_transit_gateway_vpc_attachment` using the EC2 Transit Gateway Attachment identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2transitgateway/vpcAttachment:VpcAttachment example tgw-attach-12345678
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod vpc_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcAttachmentArgs {
        /// Whether Appliance Mode support is enabled. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub appliance_mode_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether DNS support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        #[builder(into, default)]
        pub dns_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether IPv6 support is enabled. Valid values: `disable`, `enable`. Default value: `disable`.
        #[builder(into, default)]
        pub ipv6_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether Security Group Referencing Support is enabled. Valid values: `disable`, `enable`.
        #[builder(into, default)]
        pub security_group_referencing_support: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Identifiers of EC2 Subnets.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Key-value tags for the EC2 Transit Gateway VPC Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Boolean whether the VPC Attachment should be associated with the EC2 Transit Gateway association default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        #[builder(into, default)]
        pub transit_gateway_default_route_table_association: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Boolean whether the VPC Attachment should propagate routes with the EC2 Transit Gateway propagation default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        #[builder(into, default)]
        pub transit_gateway_default_route_table_propagation: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Identifier of EC2 Transit Gateway.
        #[builder(into)]
        pub transit_gateway_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of EC2 VPC.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcAttachmentResult {
        /// Whether Appliance Mode support is enabled. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. Valid values: `disable`, `enable`. Default value: `disable`.
        pub appliance_mode_support: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether DNS support is enabled. Valid values: `disable`, `enable`. Default value: `enable`.
        pub dns_support: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether IPv6 support is enabled. Valid values: `disable`, `enable`. Default value: `disable`.
        pub ipv6_support: pulumi_gestalt_rust::Output<Option<String>>,
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
        /// Boolean whether the VPC Attachment should be associated with the EC2 Transit Gateway association default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        pub transit_gateway_default_route_table_association: pulumi_gestalt_rust::Output<
            bool,
        >,
        /// Boolean whether the VPC Attachment should propagate routes with the EC2 Transit Gateway propagation default route table. This cannot be configured or perform drift detection with Resource Access Manager shared EC2 Transit Gateways. Default value: `true`.
        pub transit_gateway_default_route_table_propagation: pulumi_gestalt_rust::Output<
            bool,
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
        args: VpcAttachmentArgs,
    ) -> VpcAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let appliance_mode_support_binding = args
            .appliance_mode_support
            .get_output(context)
            .get_inner();
        let dns_support_binding = args.dns_support.get_output(context).get_inner();
        let ipv6_support_binding = args.ipv6_support.get_output(context).get_inner();
        let security_group_referencing_support_binding = args
            .security_group_referencing_support
            .get_output(context)
            .get_inner();
        let subnet_ids_binding = args.subnet_ids.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let transit_gateway_default_route_table_association_binding = args
            .transit_gateway_default_route_table_association
            .get_output(context)
            .get_inner();
        let transit_gateway_default_route_table_propagation_binding = args
            .transit_gateway_default_route_table_propagation
            .get_output(context)
            .get_inner();
        let transit_gateway_id_binding = args
            .transit_gateway_id
            .get_output(context)
            .get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/vpcAttachment:VpcAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applianceModeSupport".into(),
                    value: &appliance_mode_support_binding,
                },
                register_interface::ObjectField {
                    name: "dnsSupport".into(),
                    value: &dns_support_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6Support".into(),
                    value: &ipv6_support_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupReferencingSupport".into(),
                    value: &security_group_referencing_support_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayDefaultRouteTableAssociation".into(),
                    value: &transit_gateway_default_route_table_association_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayDefaultRouteTablePropagation".into(),
                    value: &transit_gateway_default_route_table_propagation_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcAttachmentResult {
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

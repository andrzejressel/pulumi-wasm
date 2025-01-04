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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcAttachmentAccepterArgs {
        /// Key-value tags for the EC2 Transit Gateway VPC Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the EC2 Transit Gateway Attachment to manage.
        #[builder(into)]
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
        /// Boolean whether the VPC Attachment should be associated with the EC2 Transit Gateway association default route table. Default value: `true`.
        #[builder(into, default)]
        pub transit_gateway_default_route_table_association: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Boolean whether the VPC Attachment should propagate routes with the EC2 Transit Gateway propagation default route table. Default value: `true`.
        #[builder(into, default)]
        pub transit_gateway_default_route_table_propagation: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcAttachmentAccepterResult {
        /// Whether Appliance Mode support is enabled. Valid values: `disable`, `enable`.
        pub appliance_mode_support: pulumi_wasm_rust::Output<String>,
        /// Whether DNS support is enabled. Valid values: `disable`, `enable`.
        pub dns_support: pulumi_wasm_rust::Output<String>,
        /// Whether IPv6 support is enabled. Valid values: `disable`, `enable`.
        pub ipv6_support: pulumi_wasm_rust::Output<String>,
        /// Whether Security Group Referencing Support is enabled. Valid values: `disable`, `enable`.
        pub security_group_referencing_support: pulumi_wasm_rust::Output<String>,
        /// Identifiers of EC2 Subnets.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value tags for the EC2 Transit Gateway VPC Attachment. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the EC2 Transit Gateway Attachment to manage.
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
        /// Boolean whether the VPC Attachment should be associated with the EC2 Transit Gateway association default route table. Default value: `true`.
        pub transit_gateway_default_route_table_association: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Boolean whether the VPC Attachment should propagate routes with the EC2 Transit Gateway propagation default route table. Default value: `true`.
        pub transit_gateway_default_route_table_propagation: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Identifier of EC2 Transit Gateway.
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 VPC.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the AWS account that owns the EC2 VPC.
        pub vpc_owner_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VpcAttachmentAccepterArgs,
    ) -> VpcAttachmentAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_inner();
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_inner();
        let transit_gateway_default_route_table_association_binding = args
            .transit_gateway_default_route_table_association
            .get_inner();
        let transit_gateway_default_route_table_propagation_binding = args
            .transit_gateway_default_route_table_propagation
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/vpcAttachmentAccepter:VpcAttachmentAccepter"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "applianceModeSupport".into(),
                },
                register_interface::ResultField {
                    name: "dnsSupport".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Support".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupReferencingSupport".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAttachmentId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayDefaultRouteTableAssociation".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayDefaultRouteTablePropagation".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
                register_interface::ResultField {
                    name: "vpcOwnerId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcAttachmentAccepterResult {
            appliance_mode_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applianceModeSupport").unwrap(),
            ),
            dns_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsSupport").unwrap(),
            ),
            ipv6_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Support").unwrap(),
            ),
            security_group_referencing_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupReferencingSupport").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            transit_gateway_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAttachmentId").unwrap(),
            ),
            transit_gateway_default_route_table_association: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayDefaultRouteTableAssociation").unwrap(),
            ),
            transit_gateway_default_route_table_propagation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayDefaultRouteTablePropagation").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
            vpc_owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcOwnerId").unwrap(),
            ),
        }
    }
}

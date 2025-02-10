/// Manages an EC2 Local Gateway Route Table VPC Association. More information can be found in the [Outposts User Guide](https://docs.aws.amazon.com/outposts/latest/userguide/outposts-local-gateways.html#vpc-associations).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleVpc:
///     type: aws:ec2:Vpc
///     name: example
///     properties:
///       cidrBlock: 10.0.0.0/16
///   exampleLocalGatewayRouteTableVpcAssociation:
///     type: aws:ec2:LocalGatewayRouteTableVpcAssociation
///     name: example
///     properties:
///       localGatewayRouteTableId: ${example.id}
///       vpcId: ${exampleVpc.id}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ec2:getLocalGatewayRouteTable
///       arguments:
///         outpostArn: arn:aws:outposts:us-west-2:123456789012:outpost/op-1234567890abcdef
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_local_gateway_route_table_vpc_association` using the Local Gateway Route Table VPC Association identifier. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/localGatewayRouteTableVpcAssociation:LocalGatewayRouteTableVpcAssociation example lgw-vpc-assoc-1234567890abcdef
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod local_gateway_route_table_vpc_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalGatewayRouteTableVpcAssociationArgs {
        /// Identifier of EC2 Local Gateway Route Table.
        #[builder(into)]
        pub local_gateway_route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of EC2 VPC.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LocalGatewayRouteTableVpcAssociationResult {
        pub local_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of EC2 Local Gateway Route Table.
        pub local_gateway_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifier of EC2 VPC.
        ///
        /// The following arguments are optional:
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalGatewayRouteTableVpcAssociationArgs,
    ) -> LocalGatewayRouteTableVpcAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let local_gateway_route_table_id_binding = args
            .local_gateway_route_table_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/localGatewayRouteTableVpcAssociation:LocalGatewayRouteTableVpcAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localGatewayRouteTableId".into(),
                    value: local_gateway_route_table_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocalGatewayRouteTableVpcAssociationResult {
            local_gateway_id: o.get_field("localGatewayId"),
            local_gateway_route_table_id: o.get_field("localGatewayRouteTableId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}

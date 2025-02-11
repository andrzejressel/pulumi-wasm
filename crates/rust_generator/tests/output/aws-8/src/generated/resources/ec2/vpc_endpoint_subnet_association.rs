/// Provides a resource to create an association between a VPC endpoint and a subnet.
///
/// > **NOTE on VPC Endpoints and VPC Endpoint Subnet Associations:** This provider provides
/// both a standalone VPC Endpoint Subnet Association (an association between a VPC endpoint
/// and a single `subnet_id`) and a VPC Endpoint resource with a `subnet_ids`
/// attribute. Do not use the same subnet ID in both a VPC Endpoint resource and a VPC Endpoint Subnet
/// Association resource. Doing so will cause a conflict of associations and will overwrite the association.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let snEc2 = vpc_endpoint_subnet_association::create(
///         "snEc2",
///         VpcEndpointSubnetAssociationArgs::builder()
///             .subnet_id("${sn.id}")
///             .vpc_endpoint_id("${ec2.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Endpoint Subnet Associations using `vpc_endpoint_id` together with `subnet_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcEndpointSubnetAssociation:VpcEndpointSubnetAssociation example vpce-aaaaaaaa/subnet-bbbbbbbbbbbbbbbbb
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_endpoint_subnet_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcEndpointSubnetAssociationArgs {
        /// The ID of the subnet to be associated with the VPC endpoint.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPC endpoint with which the subnet will be associated.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcEndpointSubnetAssociationResult {
        /// The ID of the subnet to be associated with the VPC endpoint.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC endpoint with which the subnet will be associated.
        pub vpc_endpoint_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcEndpointSubnetAssociationArgs,
    ) -> VpcEndpointSubnetAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let subnet_id_binding = args.subnet_id.get_output(context);
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcEndpointSubnetAssociation:VpcEndpointSubnetAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcEndpointSubnetAssociationResult {
            subnet_id: o.get_field("subnetId"),
            vpc_endpoint_id: o.get_field("vpcEndpointId"),
        }
    }
}

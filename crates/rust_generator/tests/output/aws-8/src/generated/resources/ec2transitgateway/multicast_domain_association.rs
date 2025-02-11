/// Associates the specified subnet and transit gateway attachment with the specified transit gateway multicast domain.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = transit_gateway::create(
///         "example",
///         TransitGatewayArgs::builder().multicast_support("enable").build_struct(),
///     );
///     let exampleMulticastDomain = multicast_domain::create(
///         "exampleMulticastDomain",
///         MulticastDomainArgs::builder().transit_gateway_id("${example.id}").build_struct(),
///     );
///     let exampleMulticastDomainAssociation = multicast_domain_association::create(
///         "exampleMulticastDomainAssociation",
///         MulticastDomainAssociationArgs::builder()
///             .subnet_id("${exampleAwsSubnet.id}")
///             .transit_gateway_attachment_id("${exampleVpcAttachment.id}")
///             .transit_gateway_multicast_domain_id("${exampleMulticastDomain.id}")
///             .build_struct(),
///     );
///     let exampleVpcAttachment = vpc_attachment::create(
///         "exampleVpcAttachment",
///         VpcAttachmentArgs::builder()
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .transit_gateway_id("${example.id}")
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod multicast_domain_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MulticastDomainAssociationArgs {
        /// The ID of the subnet to associate with the transit gateway multicast domain.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the transit gateway attachment.
        #[builder(into)]
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the transit gateway multicast domain.
        #[builder(into)]
        pub transit_gateway_multicast_domain_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct MulticastDomainAssociationResult {
        /// The ID of the subnet to associate with the transit gateway multicast domain.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the transit gateway attachment.
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the transit gateway multicast domain.
        pub transit_gateway_multicast_domain_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MulticastDomainAssociationArgs,
    ) -> MulticastDomainAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let subnet_id_binding = args.subnet_id.get_output(context);
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(context);
        let transit_gateway_multicast_domain_id_binding = args
            .transit_gateway_multicast_domain_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/multicastDomainAssociation:MulticastDomainAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayMulticastDomainId".into(),
                    value: &transit_gateway_multicast_domain_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MulticastDomainAssociationResult {
            subnet_id: o.get_field("subnetId"),
            transit_gateway_attachment_id: o.get_field("transitGatewayAttachmentId"),
            transit_gateway_multicast_domain_id: o
                .get_field("transitGatewayMulticastDomainId"),
        }
    }
}

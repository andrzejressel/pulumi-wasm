#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_multicast_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMulticastDomainArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetMulticastDomainFilter,
                >,
            >,
        >,
        /// Key-value tags for the EC2 Transit Gateway Multicast Domain.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the EC2 Transit Gateway Multicast Domain.
        #[builder(into, default)]
        pub transit_gateway_multicast_domain_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetMulticastDomainResult {
        /// EC2 Transit Gateway Multicast Domain ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// EC2 Transit Gateway Multicast Domain Associations
        pub associations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ec2transitgateway::GetMulticastDomainAssociation,
            >,
        >,
        /// Whether to automatically accept cross-account subnet associations that are associated with the EC2 Transit Gateway Multicast Domain.
        pub auto_accept_shared_associations: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetMulticastDomainFilter,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable Internet Group Management Protocol (IGMP) version 2 for the EC2 Transit Gateway Multicast Domain.
        pub igmpv2_support: pulumi_gestalt_rust::Output<String>,
        /// EC2 Multicast Domain Group Members
        pub members: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2transitgateway::GetMulticastDomainMember>,
        >,
        /// Identifier of the AWS account that owns the EC2 Transit Gateway Multicast Domain.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// EC2 Multicast Domain Group Sources
        pub sources: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2transitgateway::GetMulticastDomainSource>,
        >,
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable support for statically configuring multicast group sources for the EC2 Transit Gateway Multicast Domain.
        pub static_sources_support: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Multicast Domain.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The ID of the transit gateway attachment.
        pub transit_gateway_attachment_id: pulumi_gestalt_rust::Output<String>,
        /// EC2 Transit Gateway identifier.
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
        pub transit_gateway_multicast_domain_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetMulticastDomainArgs,
    ) -> GetMulticastDomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_multicast_domain_id_binding = args
            .transit_gateway_multicast_domain_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2transitgateway/getMulticastDomain:getMulticastDomain".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayMulticastDomainId".into(),
                    value: &transit_gateway_multicast_domain_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetMulticastDomainResult {
            arn: o.get_field("arn"),
            associations: o.get_field("associations"),
            auto_accept_shared_associations: o.get_field("autoAcceptSharedAssociations"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            igmpv2_support: o.get_field("igmpv2Support"),
            members: o.get_field("members"),
            owner_id: o.get_field("ownerId"),
            sources: o.get_field("sources"),
            state: o.get_field("state"),
            static_sources_support: o.get_field("staticSourcesSupport"),
            tags: o.get_field("tags"),
            transit_gateway_attachment_id: o.get_field("transitGatewayAttachmentId"),
            transit_gateway_id: o.get_field("transitGatewayId"),
            transit_gateway_multicast_domain_id: o
                .get_field("transitGatewayMulticastDomainId"),
        }
    }
}

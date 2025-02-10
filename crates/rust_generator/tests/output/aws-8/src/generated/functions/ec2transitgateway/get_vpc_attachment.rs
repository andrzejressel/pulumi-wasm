#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpc_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcAttachmentArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetVpcAttachmentFilter,
                >,
            >,
        >,
        /// Identifier of the EC2 Transit Gateway VPC Attachment.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the EC2 Transit Gateway VPC Attachment
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcAttachmentResult {
        /// Whether Appliance Mode support is enabled.
        pub appliance_mode_support: pulumi_gestalt_rust::Output<String>,
        /// Whether DNS support is enabled.
        pub dns_support: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetVpcAttachmentFilter,
                >,
            >,
        >,
        /// EC2 Transit Gateway VPC Attachment identifier
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Whether IPv6 support is enabled.
        pub ipv6_support: pulumi_gestalt_rust::Output<String>,
        /// Whether Security Group Referencing Support is enabled.
        pub security_group_referencing_support: pulumi_gestalt_rust::Output<String>,
        /// Identifiers of EC2 Subnets.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-value tags for the EC2 Transit Gateway VPC Attachment
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// EC2 Transit Gateway identifier
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
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVpcAttachmentArgs,
    ) -> GetVpcAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2transitgateway/getVpcAttachment:getVpcAttachment".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpcAttachmentResult {
            appliance_mode_support: o.get_field("applianceModeSupport"),
            dns_support: o.get_field("dnsSupport"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            ipv6_support: o.get_field("ipv6Support"),
            security_group_referencing_support: o
                .get_field("securityGroupReferencingSupport"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            transit_gateway_id: o.get_field("transitGatewayId"),
            vpc_id: o.get_field("vpcId"),
            vpc_owner_id: o.get_field("vpcOwnerId"),
        }
    }
}

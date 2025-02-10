#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_peering_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPeeringAttachmentArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetPeeringAttachmentFilter,
                >,
            >,
        >,
        /// Identifier of the EC2 Transit Gateway Peering Attachment.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match
        /// a pair on the specific EC2 Transit Gateway Peering Attachment to retrieve.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPeeringAttachmentResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetPeeringAttachmentFilter,
                >,
            >,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the peer AWS account
        pub peer_account_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the peer AWS region
        pub peer_region: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the peer EC2 Transit Gateway
        pub peer_transit_gateway_id: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Identifier of the local EC2 Transit Gateway
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPeeringAttachmentArgs,
    ) -> GetPeeringAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2transitgateway/getPeeringAttachment:getPeeringAttachment"
                .into(),
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
        GetPeeringAttachmentResult {
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            peer_account_id: o.get_field("peerAccountId"),
            peer_region: o.get_field("peerRegion"),
            peer_transit_gateway_id: o.get_field("peerTransitGatewayId"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            transit_gateway_id: o.get_field("transitGatewayId"),
        }
    }
}

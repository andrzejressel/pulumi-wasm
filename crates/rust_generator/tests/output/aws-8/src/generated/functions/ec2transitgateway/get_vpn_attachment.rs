#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpn_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpnAttachmentArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetVpnAttachmentFilter,
                >,
            >,
        >,
        /// Map of tags, each pair of which must exactly match a pair on the desired Transit Gateway VPN Attachment.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the EC2 Transit Gateway.
        #[builder(into, default)]
        pub transit_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the EC2 VPN Connection.
        #[builder(into, default)]
        pub vpn_connection_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetVpnAttachmentResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetVpnAttachmentFilter,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway VPN Attachment
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub transit_gateway_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub vpn_connection_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVpnAttachmentArgs,
    ) -> GetVpnAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_id_binding = args.transit_gateway_id.get_output(context);
        let vpn_connection_id_binding = args.vpn_connection_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2transitgateway/getVpnAttachment:getVpnAttachment".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayId".into(),
                    value: transit_gateway_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnConnectionId".into(),
                    value: vpn_connection_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpnAttachmentResult {
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            tags: o.get_field("tags"),
            transit_gateway_id: o.get_field("transitGatewayId"),
            vpn_connection_id: o.get_field("vpnConnectionId"),
        }
    }
}

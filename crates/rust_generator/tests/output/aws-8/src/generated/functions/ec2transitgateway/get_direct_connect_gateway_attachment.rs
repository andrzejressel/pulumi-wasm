#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_direct_connect_gateway_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDirectConnectGatewayAttachmentArgs {
        /// Identifier of the Direct Connect Gateway.
        #[builder(into, default)]
        pub dx_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetDirectConnectGatewayAttachmentFilter,
                >,
            >,
        >,
        /// Map of tags, each pair of which must exactly match a pair on the desired Transit Gateway Direct Connect Gateway Attachment.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the EC2 Transit Gateway.
        #[builder(into, default)]
        pub transit_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDirectConnectGatewayAttachmentResult {
        pub dx_gateway_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetDirectConnectGatewayAttachmentFilter,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Attachment
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub transit_gateway_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDirectConnectGatewayAttachmentArgs,
    ) -> GetDirectConnectGatewayAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dx_gateway_id_binding = args.dx_gateway_id.get_output(context).get_inner();
        let filters_binding = args.filters.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let transit_gateway_id_binding = args
            .transit_gateway_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2transitgateway/getDirectConnectGatewayAttachment:getDirectConnectGatewayAttachment"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dxGatewayId".into(),
                    value: &dx_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDirectConnectGatewayAttachmentResult {
            dx_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dxGatewayId"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            transit_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayId"),
            ),
        }
    }
}

#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_connect {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2transitgateway::GetConnectFilter>>,
        >,
        /// Key-value tags for the EC2 Transit Gateway Connect
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the EC2 Transit Gateway Connect.
        #[builder(into, default)]
        pub transit_gateway_connect_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConnectResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2transitgateway::GetConnectFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Tunnel protocol
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Connect
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub transit_gateway_connect_id: pulumi_gestalt_rust::Output<String>,
        /// EC2 Transit Gateway identifier
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// The underlaying VPC attachment
        pub transport_attachment_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConnectArgs,
    ) -> GetConnectResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_connect_id_binding = args
            .transit_gateway_connect_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2transitgateway/getConnect:getConnect".into(),
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
                    name: "transitGatewayConnectId".into(),
                    value: transit_gateway_connect_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConnectResult {
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            protocol: o.get_field("protocol"),
            tags: o.get_field("tags"),
            transit_gateway_connect_id: o.get_field("transitGatewayConnectId"),
            transit_gateway_id: o.get_field("transitGatewayId"),
            transport_attachment_id: o.get_field("transportAttachmentId"),
        }
    }
}

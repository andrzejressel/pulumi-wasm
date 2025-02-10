#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_local_gateway_virtual_interface_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalGatewayVirtualInterfaceGroupArgs {
        /// One or more configuration blocks containing name-values filters. See the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeLocalGatewayVirtualInterfaceGroups.html) for supported filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2::GetLocalGatewayVirtualInterfaceGroupFilter,
                >,
            >,
        >,
        /// Identifier of EC2 Local Gateway Virtual Interface Group.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of EC2 Local Gateway.
        #[builder(into, default)]
        pub local_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags, each pair of which must exactly match a pair on the desired local gateway route table.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLocalGatewayVirtualInterfaceGroupResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2::GetLocalGatewayVirtualInterfaceGroupFilter,
                >,
            >,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub local_gateway_id: pulumi_gestalt_rust::Output<String>,
        /// Set of EC2 Local Gateway Virtual Interface identifiers.
        pub local_gateway_virtual_interface_ids: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLocalGatewayVirtualInterfaceGroupArgs,
    ) -> GetLocalGatewayVirtualInterfaceGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let local_gateway_id_binding = args.local_gateway_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getLocalGatewayVirtualInterfaceGroup:getLocalGatewayVirtualInterfaceGroup"
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
                    name: "localGatewayId".into(),
                    value: local_gateway_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLocalGatewayVirtualInterfaceGroupResult {
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            local_gateway_id: o.get_field("localGatewayId"),
            local_gateway_virtual_interface_ids: o
                .get_field("localGatewayVirtualInterfaceIds"),
            tags: o.get_field("tags"),
        }
    }
}

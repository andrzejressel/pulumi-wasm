#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_local_gateway_virtual_interface_groups {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocalGatewayVirtualInterfaceGroupsArgs {
        /// One or more configuration blocks containing name-values filters. See the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeLocalGatewayVirtualInterfaceGroups.html) for supported filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2::GetLocalGatewayVirtualInterfaceGroupsFilter,
                >,
            >,
        >,
        /// Key-value map of resource tags, each pair of which must exactly match a pair on the desired local gateway route table.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLocalGatewayVirtualInterfaceGroupsResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2::GetLocalGatewayVirtualInterfaceGroupsFilter,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of EC2 Local Gateway Virtual Interface Group identifiers.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of EC2 Local Gateway Virtual Interface identifiers.
        pub local_gateway_virtual_interface_ids: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLocalGatewayVirtualInterfaceGroupsArgs,
    ) -> GetLocalGatewayVirtualInterfaceGroupsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding_1 = args.filters.get_output(context);
        let filters_binding = filters_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getLocalGatewayVirtualInterfaceGroups:getLocalGatewayVirtualInterfaceGroups"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLocalGatewayVirtualInterfaceGroupsResult {
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ids: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ids")),
            local_gateway_virtual_interface_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localGatewayVirtualInterfaceIds"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}

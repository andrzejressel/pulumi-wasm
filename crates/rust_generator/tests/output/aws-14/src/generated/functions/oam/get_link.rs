#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLinkArgs {
        /// ARN of the link.
        #[builder(into)]
        pub link_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLinkResult {
        /// ARN of the link.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Label that is assigned to this link.
        pub label: pulumi_gestalt_rust::Output<String>,
        /// Human-readable name used to identify this source account when you are viewing data from it in the monitoring account.
        pub label_template: pulumi_gestalt_rust::Output<String>,
        /// Configuration for creating filters that specify that only some metric namespaces or log groups are to be shared from the source account to the monitoring account. See `link_configuration` Block for details.
        pub link_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::oam::GetLinkLinkConfiguration>,
        >,
        /// ID string that AWS generated as part of the link ARN.
        pub link_id: pulumi_gestalt_rust::Output<String>,
        pub link_identifier: pulumi_gestalt_rust::Output<String>,
        /// Types of data that the source account shares with the monitoring account.
        pub resource_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ARN of the sink that is used for this link.
        pub sink_arn: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLinkArgs,
    ) -> GetLinkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let link_identifier_binding_1 = args.link_identifier.get_output(context);
        let link_identifier_binding = link_identifier_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:oam/getLink:getLink".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "linkIdentifier".into(),
                    value: &link_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLinkResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            label: pulumi_gestalt_rust::__private::into_domain(o.extract_field("label")),
            label_template: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labelTemplate"),
            ),
            link_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkConfigurations"),
            ),
            link_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkId"),
            ),
            link_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkIdentifier"),
            ),
            resource_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTypes"),
            ),
            sink_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sinkArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}

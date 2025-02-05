pub mod get_link {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLinkArgs {
        /// ARN of the link.
        #[builder(into)]
        pub link_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLinkResult {
        /// ARN of the link.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Label that is assigned to this link.
        pub label: pulumi_wasm_rust::Output<String>,
        /// Human-readable name used to identify this source account when you are viewing data from it in the monitoring account.
        pub label_template: pulumi_wasm_rust::Output<String>,
        /// Configuration for creating filters that specify that only some metric namespaces or log groups are to be shared from the source account to the monitoring account. See `link_configuration` Block for details.
        pub link_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::oam::GetLinkLinkConfiguration>,
        >,
        /// ID string that AWS generated as part of the link ARN.
        pub link_id: pulumi_wasm_rust::Output<String>,
        pub link_identifier: pulumi_wasm_rust::Output<String>,
        /// Types of data that the source account shares with the monitoring account.
        pub resource_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// ARN of the sink that is used for this link.
        pub sink_arn: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLinkArgs,
    ) -> GetLinkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let link_identifier_binding = args
            .link_identifier
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
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
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            label: pulumi_wasm_rust::__private::into_domain(o.extract_field("label")),
            label_template: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("labelTemplate"),
            ),
            link_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("linkConfigurations"),
            ),
            link_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("linkId")),
            link_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("linkIdentifier"),
            ),
            resource_types: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceTypes"),
            ),
            sink_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sinkArn"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}

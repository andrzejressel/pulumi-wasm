#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpc_link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcLinkArgs {
        /// Name of the API Gateway VPC Link to look up. If no API Gateway VPC Link is found with this name, an error will be returned.
        /// If multiple API Gateway VPC Links are found with this name, an error will be returned.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcLinkResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the VPC link.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Set to the ID of the found API Gateway VPC Link.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Status of the VPC link.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Status message of the VPC link.
        pub status_message: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// List of network load balancer arns in the VPC targeted by the VPC link. Currently AWS only supports 1 target.
        pub target_arns: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVpcLinkArgs,
    ) -> GetVpcLinkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getVpcLink:getVpcLink".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVpcLinkResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            status_message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusMessage"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            target_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetArns"),
            ),
        }
    }
}

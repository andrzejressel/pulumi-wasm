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
        context: &pulumi_gestalt_rust::Context,
        args: GetVpcLinkArgs,
    ) -> GetVpcLinkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:apigateway/getVpcLink:getVpcLink".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpcLinkResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            status_message: o.get_field("statusMessage"),
            tags: o.get_field("tags"),
            target_arns: o.get_field("targetArns"),
        }
    }
}

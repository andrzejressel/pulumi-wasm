#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpc_link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcLinkArgs {
        /// VPC Link Tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// VPC Link ID
        #[builder(into)]
        pub vpc_link_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVpcLinkResult {
        /// ARN of the VPC Link.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// VPC Link Name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of security groups associated with the VPC Link.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// List of subnets attached to the VPC Link.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// VPC Link Tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_link_id: pulumi_gestalt_rust::Output<String>,
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
        let tags_binding = args.tags.get_output(context);
        let vpc_link_id_binding = args.vpc_link_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:apigatewayv2/getVpcLink:getVpcLink".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcLinkId".into(),
                    value: &vpc_link_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpcLinkResult {
            arn: o.get_field("arn"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            security_group_ids: o.get_field("securityGroupIds"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            vpc_link_id: o.get_field("vpcLinkId"),
        }
    }
}

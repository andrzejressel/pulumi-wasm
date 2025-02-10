/// Manages an Amazon API Gateway Version 2 VPC Link.
///
/// > **Note:** Amazon API Gateway Version 2 VPC Links enable private integrations that connect HTTP APIs to private resources in a VPC.
/// To enable private integration for REST APIs, use the Amazon API Gateway Version 1 VPC Link resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apigatewayv2:VpcLink
///     properties:
///       name: example
///       securityGroupIds:
///         - ${exampleAwsSecurityGroup.id}
///       subnetIds: ${exampleAwsSubnets.ids}
///       tags:
///         Usage: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_vpc_link` using the VPC Link identifier. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/vpcLink:VpcLink example aabbccddee
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcLinkArgs {
        /// Name of the VPC Link. Must be between 1 and 128 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Security group IDs for the VPC Link.
        #[builder(into)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Subnet IDs for the VPC Link.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Map of tags to assign to the VPC Link. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcLinkResult {
        /// VPC Link ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the VPC Link. Must be between 1 and 128 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Security group IDs for the VPC Link.
        pub security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Subnet IDs for the VPC Link.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of tags to assign to the VPC Link. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcLinkArgs,
    ) -> VpcLinkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigatewayv2/vpcLink:VpcLink".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: security_group_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: subnet_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcLinkResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            security_group_ids: o.get_field("securityGroupIds"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}

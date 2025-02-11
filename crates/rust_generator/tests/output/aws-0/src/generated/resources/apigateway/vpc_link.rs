/// Provides an API Gateway VPC Link.
///
/// > **Note:** Amazon API Gateway Version 1 VPC Links enable private integrations that connect REST APIs to private resources in a VPC.
/// To enable private integration for HTTP APIs, use the Amazon API Gateway Version 2 VPC Link resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = load_balancer::create(
///         "example",
///         LoadBalancerArgs::builder()
///             .internal(true)
///             .load_balancer_type("network")
///             .name("example")
///             .subnet_mappings(
///                 vec![
///                     LoadBalancerSubnetMapping::builder().subnetId("12345")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleVpcLink = vpc_link::create(
///         "exampleVpcLink",
///         VpcLinkArgs::builder()
///             .description("example description")
///             .name("example")
///             .target_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import API Gateway VPC Link using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/vpcLink:VpcLink example 12345abcde
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcLinkArgs {
        /// Description of the VPC link.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name used to label and identify the VPC link.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of network load balancer arns in the VPC targeted by the VPC link. Currently AWS only supports 1 target.
        #[builder(into)]
        pub target_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcLinkResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the VPC link.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name used to label and identify the VPC link.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of network load balancer arns in the VPC targeted by the VPC link. Currently AWS only supports 1 target.
        pub target_arn: pulumi_gestalt_rust::Output<String>,
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
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_arn_binding = args.target_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/vpcLink:VpcLink".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetArn".into(),
                    value: &target_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcLinkResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_arn: o.get_field("targetArn"),
        }
    }
}

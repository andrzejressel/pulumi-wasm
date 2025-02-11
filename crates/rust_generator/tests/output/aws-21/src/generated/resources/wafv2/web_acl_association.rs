/// Creates a WAFv2 Web ACL Association.
///
/// > **NOTE on associating a WAFv2 Web ACL with a Cloudfront distribution:** Do not use this resource to associate a WAFv2 Web ACL with a Cloudfront Distribution. The [AWS API call backing this resource](https://docs.aws.amazon.com/waf/latest/APIReference/API_AssociateWebACL.html) notes that you should use the `web_acl_id` property on the `cloudfront_distribution` instead.
///
/// [1]: https://docs.aws.amazon.com/waf/latest/APIReference/API_AssociateWebACL.html
///
///
/// ## Import
///
/// Using `pulumi import`, import WAFv2 Web ACL Association using `WEB_ACL_ARN,RESOURCE_ARN`. For example:
///
/// ```sh
/// $ pulumi import aws:wafv2/webAclAssociation:WebAclAssociation example arn:aws:wafv2:...7ce849ea,arn:aws:apigateway:...ages/name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_acl_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAclAssociationArgs {
        /// The Amazon Resource Name (ARN) of the resource to associate with the web ACL. This must be an ARN of an Application Load Balancer, an Amazon API Gateway stage (REST only, HTTP is unsupported), an Amazon Cognito User Pool, an Amazon AppSync GraphQL API, an Amazon App Runner service, or an Amazon Verified Access instance.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the Web ACL that you want to associate with the resource.
        #[builder(into)]
        pub web_acl_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WebAclAssociationResult {
        /// The Amazon Resource Name (ARN) of the resource to associate with the web ACL. This must be an ARN of an Application Load Balancer, an Amazon API Gateway stage (REST only, HTTP is unsupported), an Amazon Cognito User Pool, an Amazon AppSync GraphQL API, an Amazon App Runner service, or an Amazon Verified Access instance.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Web ACL that you want to associate with the resource.
        pub web_acl_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebAclAssociationArgs,
    ) -> WebAclAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resource_arn_binding = args.resource_arn.get_output(context);
        let web_acl_arn_binding = args.web_acl_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:wafv2/webAclAssociation:WebAclAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webAclArn".into(),
                    value: &web_acl_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebAclAssociationResult {
            resource_arn: o.get_field("resourceArn"),
            web_acl_arn: o.get_field("webAclArn"),
        }
    }
}

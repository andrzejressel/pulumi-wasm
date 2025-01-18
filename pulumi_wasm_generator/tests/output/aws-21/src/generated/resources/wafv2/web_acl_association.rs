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
pub mod web_acl_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAclAssociationArgs {
        /// The Amazon Resource Name (ARN) of the resource to associate with the web ACL. This must be an ARN of an Application Load Balancer, an Amazon API Gateway stage (REST only, HTTP is unsupported), an Amazon Cognito User Pool, an Amazon AppSync GraphQL API, an Amazon App Runner service, or an Amazon Verified Access instance.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Web ACL that you want to associate with the resource.
        #[builder(into)]
        pub web_acl_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WebAclAssociationResult {
        /// The Amazon Resource Name (ARN) of the resource to associate with the web ACL. This must be an ARN of an Application Load Balancer, an Amazon API Gateway stage (REST only, HTTP is unsupported), an Amazon Cognito User Pool, an Amazon AppSync GraphQL API, an Amazon App Runner service, or an Amazon Verified Access instance.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Web ACL that you want to associate with the resource.
        pub web_acl_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WebAclAssociationArgs) -> WebAclAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_arn_binding = args.resource_arn.get_inner();
        let web_acl_arn_binding = args.web_acl_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafv2/webAclAssociation:WebAclAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
                register_interface::ObjectField {
                    name: "webAclArn".into(),
                    value: &web_acl_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "webAclArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WebAclAssociationResult {
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            web_acl_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webAclArn").unwrap(),
            ),
        }
    }
}

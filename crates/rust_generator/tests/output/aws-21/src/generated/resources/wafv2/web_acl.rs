#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_acl {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAclArgs {
        /// Specifies custom configurations for the associations between the web ACL and protected resources. See `association_config` below for details.
        #[builder(into, default)]
        pub association_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::wafv2::WebAclAssociationConfig>,
        >,
        /// Specifies how AWS WAF should handle CAPTCHA evaluations on the ACL level (used by [AWS Bot Control](https://docs.aws.amazon.com/waf/latest/developerguide/aws-managed-rule-groups-bot.html)). See `captcha_config` below for details.
        #[builder(into, default)]
        pub captcha_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::wafv2::WebAclCaptchaConfig>,
        >,
        /// Specifies how AWS WAF should handle Challenge evaluations on the ACL level (used by [AWS Bot Control](https://docs.aws.amazon.com/waf/latest/developerguide/aws-managed-rule-groups-bot.html)). See `challenge_config` below for details.
        #[builder(into, default)]
        pub challenge_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::wafv2::WebAclChallengeConfig>,
        >,
        /// Defines custom response bodies that can be referenced by `custom_response` actions. See `custom_response_body` below for details.
        #[builder(into, default)]
        pub custom_response_bodies: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::wafv2::WebAclCustomResponseBody>>,
        >,
        /// Action to perform if none of the `rules` contained in the WebACL match. See `default_action` below for details.
        #[builder(into)]
        pub default_action: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::wafv2::WebAclDefaultAction,
        >,
        /// Friendly description of the WebACL.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Friendly name of the WebACL.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Raw JSON string to allow more than three nested statements. Conflicts with `rule` attribute. This is for advanced use cases where more than 3 levels of nested statements are required. **There is no drift detection at this time**. If you use this attribute instead of `rule`, you will be foregoing drift detection. See the AWS [documentation](https://docs.aws.amazon.com/waf/latest/APIReference/API_CreateWebACL.html) for the JSON structure.
        #[builder(into, default)]
        pub rule_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Rule blocks used to identify the web requests that you want to `allow`, `block`, or `count`. See `rule` below for details.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::wafv2::WebAclRule>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of key-value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the domains that AWS WAF should accept in a web request token. This enables the use of tokens across multiple protected websites. When AWS WAF provides a token, it uses the domain of the AWS resource that the web ACL is protecting. If you don't specify a list of token domains, AWS WAF accepts tokens only for the domain of the protected resource. With a token domain list, AWS WAF accepts the resource's host domain plus all domains in the token domain list, including their prefixed subdomains.
        #[builder(into, default)]
        pub token_domains: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Defines and enables Amazon CloudWatch metrics and web request sample collection. See `visibility_config` below for details.
        #[builder(into)]
        pub visibility_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::wafv2::WebAclVisibilityConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct WebAclResult {
        /// The URL to use in SDK integrations with managed rule groups.
        pub application_integration_url: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the WAF WebACL.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies custom configurations for the associations between the web ACL and protected resources. See `association_config` below for details.
        pub association_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::wafv2::WebAclAssociationConfig>,
        >,
        /// Web ACL capacity units (WCUs) currently being used by this web ACL.
        pub capacity: pulumi_gestalt_rust::Output<i32>,
        /// Specifies how AWS WAF should handle CAPTCHA evaluations on the ACL level (used by [AWS Bot Control](https://docs.aws.amazon.com/waf/latest/developerguide/aws-managed-rule-groups-bot.html)). See `captcha_config` below for details.
        pub captcha_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::wafv2::WebAclCaptchaConfig>,
        >,
        /// Specifies how AWS WAF should handle Challenge evaluations on the ACL level (used by [AWS Bot Control](https://docs.aws.amazon.com/waf/latest/developerguide/aws-managed-rule-groups-bot.html)). See `challenge_config` below for details.
        pub challenge_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::wafv2::WebAclChallengeConfig>,
        >,
        /// Defines custom response bodies that can be referenced by `custom_response` actions. See `custom_response_body` below for details.
        pub custom_response_bodies: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::wafv2::WebAclCustomResponseBody>>,
        >,
        /// Action to perform if none of the `rules` contained in the WebACL match. See `default_action` below for details.
        pub default_action: pulumi_gestalt_rust::Output<
            super::super::types::wafv2::WebAclDefaultAction,
        >,
        /// Friendly description of the WebACL.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub lock_token: pulumi_gestalt_rust::Output<String>,
        /// Friendly name of the WebACL.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Raw JSON string to allow more than three nested statements. Conflicts with `rule` attribute. This is for advanced use cases where more than 3 levels of nested statements are required. **There is no drift detection at this time**. If you use this attribute instead of `rule`, you will be foregoing drift detection. See the AWS [documentation](https://docs.aws.amazon.com/waf/latest/APIReference/API_CreateWebACL.html) for the JSON structure.
        pub rule_json: pulumi_gestalt_rust::Output<Option<String>>,
        /// Rule blocks used to identify the web requests that you want to `allow`, `block`, or `count`. See `rule` below for details.
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::wafv2::WebAclRule>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// Map of key-value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the domains that AWS WAF should accept in a web request token. This enables the use of tokens across multiple protected websites. When AWS WAF provides a token, it uses the domain of the AWS resource that the web ACL is protecting. If you don't specify a list of token domains, AWS WAF accepts tokens only for the domain of the protected resource. With a token domain list, AWS WAF accepts the resource's host domain plus all domains in the token domain list, including their prefixed subdomains.
        pub token_domains: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Defines and enables Amazon CloudWatch metrics and web request sample collection. See `visibility_config` below for details.
        pub visibility_config: pulumi_gestalt_rust::Output<
            super::super::types::wafv2::WebAclVisibilityConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebAclArgs,
    ) -> WebAclResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let association_config_binding = args.association_config.get_output(context);
        let captcha_config_binding = args.captcha_config.get_output(context);
        let challenge_config_binding = args.challenge_config.get_output(context);
        let custom_response_bodies_binding = args
            .custom_response_bodies
            .get_output(context);
        let default_action_binding = args.default_action.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let rule_json_binding = args.rule_json.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let token_domains_binding = args.token_domains.get_output(context);
        let visibility_config_binding = args.visibility_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:wafv2/webAcl:WebAcl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "associationConfig".into(),
                    value: association_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "captchaConfig".into(),
                    value: captcha_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "challengeConfig".into(),
                    value: challenge_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customResponseBodies".into(),
                    value: custom_response_bodies_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultAction".into(),
                    value: default_action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleJson".into(),
                    value: rule_json_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tokenDomains".into(),
                    value: token_domains_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "visibilityConfig".into(),
                    value: visibility_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebAclResult {
            application_integration_url: o.get_field("applicationIntegrationUrl"),
            arn: o.get_field("arn"),
            association_config: o.get_field("associationConfig"),
            capacity: o.get_field("capacity"),
            captcha_config: o.get_field("captchaConfig"),
            challenge_config: o.get_field("challengeConfig"),
            custom_response_bodies: o.get_field("customResponseBodies"),
            default_action: o.get_field("defaultAction"),
            description: o.get_field("description"),
            lock_token: o.get_field("lockToken"),
            name: o.get_field("name"),
            rule_json: o.get_field("ruleJson"),
            rules: o.get_field("rules"),
            scope: o.get_field("scope"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            token_domains: o.get_field("tokenDomains"),
            visibility_config: o.get_field("visibilityConfig"),
        }
    }
}

pub mod web_acl {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAclArgs {
        /// Specifies custom configurations for the associations between the web ACL and protected resources. See `association_config` below for details.
        #[builder(into, default)]
        pub association_config: pulumi_wasm_rust::Output<
            Option<super::super::types::wafv2::WebAclAssociationConfig>,
        >,
        /// Specifies how AWS WAF should handle CAPTCHA evaluations on the ACL level (used by [AWS Bot Control](https://docs.aws.amazon.com/waf/latest/developerguide/aws-managed-rule-groups-bot.html)). See `captcha_config` below for details.
        #[builder(into, default)]
        pub captcha_config: pulumi_wasm_rust::Output<
            Option<super::super::types::wafv2::WebAclCaptchaConfig>,
        >,
        /// Specifies how AWS WAF should handle Challenge evaluations on the ACL level (used by [AWS Bot Control](https://docs.aws.amazon.com/waf/latest/developerguide/aws-managed-rule-groups-bot.html)). See `challenge_config` below for details.
        #[builder(into, default)]
        pub challenge_config: pulumi_wasm_rust::Output<
            Option<super::super::types::wafv2::WebAclChallengeConfig>,
        >,
        /// Defines custom response bodies that can be referenced by `custom_response` actions. See `custom_response_body` below for details.
        #[builder(into, default)]
        pub custom_response_bodies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafv2::WebAclCustomResponseBody>>,
        >,
        /// Action to perform if none of the `rules` contained in the WebACL match. See `default_action` below for details.
        #[builder(into)]
        pub default_action: pulumi_wasm_rust::Output<
            super::super::types::wafv2::WebAclDefaultAction,
        >,
        /// Friendly description of the WebACL.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Friendly name of the WebACL.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Raw JSON string to allow more than three nested statements. Conflicts with `rule` attribute. This is for advanced use cases where more than 3 levels of nested statements are required. **There is no drift detection at this time**. If you use this attribute instead of `rule`, you will be foregoing drift detection. See the AWS [documentation](https://docs.aws.amazon.com/waf/latest/APIReference/API_CreateWebACL.html) for the JSON structure.
        #[builder(into, default)]
        pub rule_json: pulumi_wasm_rust::Output<Option<String>>,
        /// Rule blocks used to identify the web requests that you want to `allow`, `block`, or `count`. See `rule` below for details.
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafv2::WebAclRule>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<String>,
        /// Map of key-value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the domains that AWS WAF should accept in a web request token. This enables the use of tokens across multiple protected websites. When AWS WAF provides a token, it uses the domain of the AWS resource that the web ACL is protecting. If you don't specify a list of token domains, AWS WAF accepts tokens only for the domain of the protected resource. With a token domain list, AWS WAF accepts the resource's host domain plus all domains in the token domain list, including their prefixed subdomains.
        #[builder(into, default)]
        pub token_domains: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Defines and enables Amazon CloudWatch metrics and web request sample collection. See `visibility_config` below for details.
        #[builder(into)]
        pub visibility_config: pulumi_wasm_rust::Output<
            super::super::types::wafv2::WebAclVisibilityConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct WebAclResult {
        /// The URL to use in SDK integrations with managed rule groups.
        pub application_integration_url: pulumi_wasm_rust::Output<String>,
        /// The ARN of the WAF WebACL.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies custom configurations for the associations between the web ACL and protected resources. See `association_config` below for details.
        pub association_config: pulumi_wasm_rust::Output<
            Option<super::super::types::wafv2::WebAclAssociationConfig>,
        >,
        /// Web ACL capacity units (WCUs) currently being used by this web ACL.
        pub capacity: pulumi_wasm_rust::Output<i32>,
        /// Specifies how AWS WAF should handle CAPTCHA evaluations on the ACL level (used by [AWS Bot Control](https://docs.aws.amazon.com/waf/latest/developerguide/aws-managed-rule-groups-bot.html)). See `captcha_config` below for details.
        pub captcha_config: pulumi_wasm_rust::Output<
            Option<super::super::types::wafv2::WebAclCaptchaConfig>,
        >,
        /// Specifies how AWS WAF should handle Challenge evaluations on the ACL level (used by [AWS Bot Control](https://docs.aws.amazon.com/waf/latest/developerguide/aws-managed-rule-groups-bot.html)). See `challenge_config` below for details.
        pub challenge_config: pulumi_wasm_rust::Output<
            Option<super::super::types::wafv2::WebAclChallengeConfig>,
        >,
        /// Defines custom response bodies that can be referenced by `custom_response` actions. See `custom_response_body` below for details.
        pub custom_response_bodies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafv2::WebAclCustomResponseBody>>,
        >,
        /// Action to perform if none of the `rules` contained in the WebACL match. See `default_action` below for details.
        pub default_action: pulumi_wasm_rust::Output<
            super::super::types::wafv2::WebAclDefaultAction,
        >,
        /// Friendly description of the WebACL.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub lock_token: pulumi_wasm_rust::Output<String>,
        /// Friendly name of the WebACL.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Raw JSON string to allow more than three nested statements. Conflicts with `rule` attribute. This is for advanced use cases where more than 3 levels of nested statements are required. **There is no drift detection at this time**. If you use this attribute instead of `rule`, you will be foregoing drift detection. See the AWS [documentation](https://docs.aws.amazon.com/waf/latest/APIReference/API_CreateWebACL.html) for the JSON structure.
        pub rule_json: pulumi_wasm_rust::Output<Option<String>>,
        /// Rule blocks used to identify the web requests that you want to `allow`, `block`, or `count`. See `rule` below for details.
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafv2::WebAclRule>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        pub scope: pulumi_wasm_rust::Output<String>,
        /// Map of key-value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the domains that AWS WAF should accept in a web request token. This enables the use of tokens across multiple protected websites. When AWS WAF provides a token, it uses the domain of the AWS resource that the web ACL is protecting. If you don't specify a list of token domains, AWS WAF accepts tokens only for the domain of the protected resource. With a token domain list, AWS WAF accepts the resource's host domain plus all domains in the token domain list, including their prefixed subdomains.
        pub token_domains: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Defines and enables Amazon CloudWatch metrics and web request sample collection. See `visibility_config` below for details.
        pub visibility_config: pulumi_wasm_rust::Output<
            super::super::types::wafv2::WebAclVisibilityConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WebAclArgs) -> WebAclResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let association_config_binding = args.association_config.get_inner();
        let captcha_config_binding = args.captcha_config.get_inner();
        let challenge_config_binding = args.challenge_config.get_inner();
        let custom_response_bodies_binding = args.custom_response_bodies.get_inner();
        let default_action_binding = args.default_action.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let rule_json_binding = args.rule_json.get_inner();
        let rules_binding = args.rules.get_inner();
        let scope_binding = args.scope.get_inner();
        let tags_binding = args.tags.get_inner();
        let token_domains_binding = args.token_domains.get_inner();
        let visibility_config_binding = args.visibility_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafv2/webAcl:WebAcl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "associationConfig".into(),
                    value: &association_config_binding,
                },
                register_interface::ObjectField {
                    name: "captchaConfig".into(),
                    value: &captcha_config_binding,
                },
                register_interface::ObjectField {
                    name: "challengeConfig".into(),
                    value: &challenge_config_binding,
                },
                register_interface::ObjectField {
                    name: "customResponseBodies".into(),
                    value: &custom_response_bodies_binding,
                },
                register_interface::ObjectField {
                    name: "defaultAction".into(),
                    value: &default_action_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ruleJson".into(),
                    value: &rule_json_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tokenDomains".into(),
                    value: &token_domains_binding,
                },
                register_interface::ObjectField {
                    name: "visibilityConfig".into(),
                    value: &visibility_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationIntegrationUrl".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associationConfig".into(),
                },
                register_interface::ResultField {
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "captchaConfig".into(),
                },
                register_interface::ResultField {
                    name: "challengeConfig".into(),
                },
                register_interface::ResultField {
                    name: "customResponseBodies".into(),
                },
                register_interface::ResultField {
                    name: "defaultAction".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "lockToken".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ruleJson".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "tokenDomains".into(),
                },
                register_interface::ResultField {
                    name: "visibilityConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WebAclResult {
            application_integration_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationIntegrationUrl").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            association_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationConfig").unwrap(),
            ),
            capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacity").unwrap(),
            ),
            captcha_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("captchaConfig").unwrap(),
            ),
            challenge_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("challengeConfig").unwrap(),
            ),
            custom_response_bodies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customResponseBodies").unwrap(),
            ),
            default_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultAction").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            lock_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lockToken").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rule_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleJson").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            token_domains: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tokenDomains").unwrap(),
            ),
            visibility_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("visibilityConfig").unwrap(),
            ),
        }
    }
}

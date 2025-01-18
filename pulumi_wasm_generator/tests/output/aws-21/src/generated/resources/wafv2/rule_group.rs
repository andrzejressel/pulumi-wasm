pub mod rule_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleGroupArgs {
        /// The web ACL capacity units (WCUs) required for this rule group. See [here](https://docs.aws.amazon.com/waf/latest/APIReference/API_CreateRuleGroup.html#API_CreateRuleGroup_RequestSyntax) for general information and [here](https://docs.aws.amazon.com/waf/latest/developerguide/waf-rule-statements-list.html) for capacity specific information.
        #[builder(into)]
        pub capacity: pulumi_wasm_rust::Output<i32>,
        /// Defines custom response bodies that can be referenced by `custom_response` actions. See Custom Response Body below for details.
        #[builder(into, default)]
        pub custom_response_bodies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafv2::RuleGroupCustomResponseBody>>,
        >,
        /// A friendly description of the rule group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A friendly name of the rule group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The rule blocks used to identify the web requests that you want to `allow`, `block`, or `count`. See Rules below for details.
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafv2::RuleGroupRule>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::Output<String>,
        /// An array of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Defines and enables Amazon CloudWatch metrics and web request sample collection. See Visibility Configuration below for details.
        #[builder(into)]
        pub visibility_config: pulumi_wasm_rust::Output<
            super::super::types::wafv2::RuleGroupVisibilityConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct RuleGroupResult {
        /// The ARN of the WAF rule group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The web ACL capacity units (WCUs) required for this rule group. See [here](https://docs.aws.amazon.com/waf/latest/APIReference/API_CreateRuleGroup.html#API_CreateRuleGroup_RequestSyntax) for general information and [here](https://docs.aws.amazon.com/waf/latest/developerguide/waf-rule-statements-list.html) for capacity specific information.
        pub capacity: pulumi_wasm_rust::Output<i32>,
        /// Defines custom response bodies that can be referenced by `custom_response` actions. See Custom Response Body below for details.
        pub custom_response_bodies: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafv2::RuleGroupCustomResponseBody>>,
        >,
        /// A friendly description of the rule group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub lock_token: pulumi_wasm_rust::Output<String>,
        /// A friendly name of the rule group.
        pub name: pulumi_wasm_rust::Output<String>,
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// The rule blocks used to identify the web requests that you want to `allow`, `block`, or `count`. See Rules below for details.
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafv2::RuleGroupRule>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        pub scope: pulumi_wasm_rust::Output<String>,
        /// An array of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Defines and enables Amazon CloudWatch metrics and web request sample collection. See Visibility Configuration below for details.
        pub visibility_config: pulumi_wasm_rust::Output<
            super::super::types::wafv2::RuleGroupVisibilityConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RuleGroupArgs) -> RuleGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_binding = args.capacity.get_inner();
        let custom_response_bodies_binding = args.custom_response_bodies.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let rules_binding = args.rules.get_inner();
        let scope_binding = args.scope.get_inner();
        let tags_binding = args.tags.get_inner();
        let visibility_config_binding = args.visibility_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafv2/ruleGroup:RuleGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding,
                },
                register_interface::ObjectField {
                    name: "customResponseBodies".into(),
                    value: &custom_response_bodies_binding,
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
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
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
                    name: "visibilityConfig".into(),
                    value: &visibility_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "customResponseBodies".into(),
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
                    name: "namePrefix".into(),
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
        RuleGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacity").unwrap(),
            ),
            custom_response_bodies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customResponseBodies").unwrap(),
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
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
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
            visibility_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("visibilityConfig").unwrap(),
            ),
        }
    }
}

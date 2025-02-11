#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod rule_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleGroupArgs {
        /// The web ACL capacity units (WCUs) required for this rule group. See [here](https://docs.aws.amazon.com/waf/latest/APIReference/API_CreateRuleGroup.html#API_CreateRuleGroup_RequestSyntax) for general information and [here](https://docs.aws.amazon.com/waf/latest/developerguide/waf-rule-statements-list.html) for capacity specific information.
        #[builder(into)]
        pub capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Defines custom response bodies that can be referenced by `custom_response` actions. See Custom Response Body below for details.
        #[builder(into, default)]
        pub custom_response_bodies: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::wafv2::RuleGroupCustomResponseBody>>,
        >,
        /// A friendly description of the rule group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A friendly name of the rule group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The rule blocks used to identify the web requests that you want to `allow`, `block`, or `count`. See Rules below for details.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::wafv2::RuleGroupRule>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An array of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Defines and enables Amazon CloudWatch metrics and web request sample collection. See Visibility Configuration below for details.
        #[builder(into)]
        pub visibility_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::wafv2::RuleGroupVisibilityConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct RuleGroupResult {
        /// The ARN of the WAF rule group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The web ACL capacity units (WCUs) required for this rule group. See [here](https://docs.aws.amazon.com/waf/latest/APIReference/API_CreateRuleGroup.html#API_CreateRuleGroup_RequestSyntax) for general information and [here](https://docs.aws.amazon.com/waf/latest/developerguide/waf-rule-statements-list.html) for capacity specific information.
        pub capacity: pulumi_gestalt_rust::Output<i32>,
        /// Defines custom response bodies that can be referenced by `custom_response` actions. See Custom Response Body below for details.
        pub custom_response_bodies: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::wafv2::RuleGroupCustomResponseBody>>,
        >,
        /// A friendly description of the rule group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub lock_token: pulumi_gestalt_rust::Output<String>,
        /// A friendly name of the rule group.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The rule blocks used to identify the web requests that you want to `allow`, `block`, or `count`. See Rules below for details.
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::wafv2::RuleGroupRule>>,
        >,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// An array of key:value pairs to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Defines and enables Amazon CloudWatch metrics and web request sample collection. See Visibility Configuration below for details.
        pub visibility_config: pulumi_gestalt_rust::Output<
            super::super::types::wafv2::RuleGroupVisibilityConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RuleGroupArgs,
    ) -> RuleGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capacity_binding = args.capacity.get_output(context);
        let custom_response_bodies_binding = args
            .custom_response_bodies
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let visibility_config_binding = args.visibility_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:wafv2/ruleGroup:RuleGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customResponseBodies".into(),
                    value: &custom_response_bodies_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: &scope_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "visibilityConfig".into(),
                    value: &visibility_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RuleGroupResult {
            arn: o.get_field("arn"),
            capacity: o.get_field("capacity"),
            custom_response_bodies: o.get_field("customResponseBodies"),
            description: o.get_field("description"),
            lock_token: o.get_field("lockToken"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            rules: o.get_field("rules"),
            scope: o.get_field("scope"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            visibility_config: o.get_field("visibilityConfig"),
        }
    }
}

/// Provides a WAF Rule Group Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = rule::create(
///         "example",
///         RuleArgs::builder().metric_name("example").name("example").build_struct(),
///     );
///     let exampleRuleGroup = rule_group::create(
///         "exampleRuleGroup",
///         RuleGroupArgs::builder()
///             .activated_rules(
///                 vec![
///                     RuleGroupActivatedRule::builder()
///                     .action(RuleGroupActivatedRuleAction::builder(). type ("COUNT")
///                     .build_struct()).priority(50).ruleId("${example.id}").build_struct(),
///                 ],
///             )
///             .metric_name("example")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF Rule Group using the id. For example:
///
/// ```sh
/// $ pulumi import aws:waf/ruleGroup:RuleGroup example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod rule_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleGroupArgs {
        /// A list of activated rules, see below
        #[builder(into, default)]
        pub activated_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::waf::RuleGroupActivatedRule>>,
        >,
        /// A friendly name for the metrics from the rule group
        #[builder(into)]
        pub metric_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the rule group. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RuleGroupResult {
        /// A list of activated rules, see below
        pub activated_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::waf::RuleGroupActivatedRule>>,
        >,
        /// The ARN of the WAF rule group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A friendly name for the metrics from the rule group
        pub metric_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the rule group. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RuleGroupArgs,
    ) -> RuleGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let activated_rules_binding = args
            .activated_rules
            .get_output(context)
            .get_inner();
        let metric_name_binding = args.metric_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:waf/ruleGroup:RuleGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activatedRules".into(),
                    value: &activated_rules_binding,
                },
                register_interface::ObjectField {
                    name: "metricName".into(),
                    value: &metric_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RuleGroupResult {
            activated_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("activatedRules"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            metric_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metricName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}

/// Provides a WAF Regional Rule Group Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// Using `pulumi import`, import WAF Regional Rule Group using the id. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/ruleGroup:RuleGroup example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
pub mod rule_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleGroupArgs {
        /// A list of activated rules, see below
        #[builder(into, default)]
        pub activated_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafregional::RuleGroupActivatedRule>>,
        >,
        /// A friendly name for the metrics from the rule group
        #[builder(into)]
        pub metric_name: pulumi_wasm_rust::Output<String>,
        /// A friendly name of the rule group
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RuleGroupResult {
        /// A list of activated rules, see below
        pub activated_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::wafregional::RuleGroupActivatedRule>>,
        >,
        /// The ARN of the WAF Regional Rule Group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A friendly name for the metrics from the rule group
        pub metric_name: pulumi_wasm_rust::Output<String>,
        /// A friendly name of the rule group
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RuleGroupArgs) -> RuleGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let activated_rules_binding = args.activated_rules.get_inner();
        let metric_name_binding = args.metric_name.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafregional/ruleGroup:RuleGroup".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "activatedRules".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "metricName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
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
            activated_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activatedRules").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            metric_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}

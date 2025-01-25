/// Provides a WAF Rule Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ipset = ip_set::create(
///         "ipset",
///         IpSetArgs::builder()
///             .ip_set_descriptors(
///                 vec![
///                     IpSetIpSetDescriptor::builder(). type ("IPV4").value("192.0.7.0/24")
///                     .build_struct(),
///                 ],
///             )
///             .name("tfIPSet")
///             .build_struct(),
///     );
///     let wafrule = rule::create(
///         "wafrule",
///         RuleArgs::builder()
///             .metric_name("tfWAFRule")
///             .name("tfWAFRule")
///             .predicates(
///                 vec![
///                     RulePredicate::builder().dataId("${ipset.id}").negated(false). type
///                     ("IPMatch").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF rules using the id. For example:
///
/// ```sh
/// $ pulumi import aws:waf/rule:Rule example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
pub mod rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleArgs {
        /// The name or description for the Amazon CloudWatch metric of this rule. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace.
        #[builder(into)]
        pub metric_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name or description of the rule.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The objects to include in a rule (documented below).
        #[builder(into, default)]
        pub predicates: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::waf::RulePredicate>>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RuleResult {
        /// The ARN of the WAF rule.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name or description for the Amazon CloudWatch metric of this rule. The name can contain only alphanumeric characters (A-Z, a-z, 0-9); the name can't contain whitespace.
        pub metric_name: pulumi_wasm_rust::Output<String>,
        /// The name or description of the rule.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The objects to include in a rule (documented below).
        pub predicates: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::waf::RulePredicate>>,
        >,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RuleArgs,
    ) -> RuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let metric_name_binding = args.metric_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let predicates_binding = args.predicates.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:waf/rule:Rule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "metricName".into(),
                    value: &metric_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "predicates".into(),
                    value: &predicates_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
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
                    name: "predicates".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            metric_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            predicates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("predicates").unwrap(),
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

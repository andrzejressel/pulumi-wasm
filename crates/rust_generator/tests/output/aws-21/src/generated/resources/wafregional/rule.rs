/// Provides an WAF Regional Rule Resource for use with Application Load Balancer.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// ## Nested Fields
///
/// ### `predicate`
///
/// See the [WAF Documentation](https://docs.aws.amazon.com/waf/latest/APIReference/API_Predicate.html) for more information.
///
/// #### Arguments
///
/// * `type` - (Required) The type of predicate in a rule. Valid values: `ByteMatch`, `GeoMatch`, `IPMatch`, `RegexMatch`, `SizeConstraint`, `SqlInjectionMatch`, or `XssMatch`
/// * `data_id` - (Required) The unique identifier of a predicate, such as the ID of a `ByteMatchSet` or `IPSet`.
/// * `negated` - (Required) Whether to use the settings or the negated settings that you specified in the objects.
///
/// ## Import
///
/// Using `pulumi import`, import WAF Regional Rule using the id. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/rule:Rule wafrule a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleArgs {
        /// The name or description for the Amazon CloudWatch metric of this rule.
        #[builder(into)]
        pub metric_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name or description of the rule.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The objects to include in a rule (documented below).
        #[builder(into, default)]
        pub predicates: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::wafregional::RulePredicate>>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RuleResult {
        /// The ARN of the WAF Regional Rule.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name or description for the Amazon CloudWatch metric of this rule.
        pub metric_name: pulumi_gestalt_rust::Output<String>,
        /// The name or description of the rule.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The objects to include in a rule (documented below).
        pub predicates: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::wafregional::RulePredicate>>,
        >,
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
        args: RuleArgs,
    ) -> RuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let metric_name_binding_1 = args.metric_name.get_output(context);
        let metric_name_binding = metric_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let predicates_binding_1 = args.predicates.get_output(context);
        let predicates_binding = predicates_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafregional/rule:Rule".into(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        RuleResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            metric_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metricName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            predicates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("predicates"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}

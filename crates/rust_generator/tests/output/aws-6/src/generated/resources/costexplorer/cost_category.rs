/// Provides a CE Cost Category.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = cost_category::create(
///         "test",
///         CostCategoryArgs::builder()
///             .name("NAME")
///             .rule_version("CostCategoryExpression.v1")
///             .rules(
///                 vec![
///                     CostCategoryRule::builder().rule(CostCategoryRuleRule::builder()
///                     .dimension(CostCategoryRuleRuleDimension::builder()
///                     .key("LINKED_ACCOUNT_NAME").matchOptions(vec!["ENDS_WITH",])
///                     .values(vec!["-prod",]).build_struct()).build_struct())
///                     .value("production").build_struct(), CostCategoryRule::builder()
///                     .rule(CostCategoryRuleRule::builder()
///                     .dimension(CostCategoryRuleRuleDimension::builder()
///                     .key("LINKED_ACCOUNT_NAME").matchOptions(vec!["ENDS_WITH",])
///                     .values(vec!["-stg",]).build_struct()).build_struct())
///                     .value("staging").build_struct(), CostCategoryRule::builder()
///                     .rule(CostCategoryRuleRule::builder()
///                     .dimension(CostCategoryRuleRuleDimension::builder()
///                     .key("LINKED_ACCOUNT_NAME").matchOptions(vec!["ENDS_WITH",])
///                     .values(vec!["-dev",]).build_struct()).build_struct())
///                     .value("testing").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ce_cost_category` using the id. For example:
///
/// ```sh
/// $ pulumi import aws:costexplorer/costCategory:CostCategory example costCategoryARN
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cost_category {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CostCategoryArgs {
        /// Default value for the cost category.
        #[builder(into, default)]
        pub default_value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Cost Category's effective start date. It can only be a billing start date (first day of the month). If the date isn't provided, it's the first day of the current month. Dates can't be before the previous twelve months, or in the future. For example `2022-11-01T00:00:00Z`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub effective_start: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name for the Cost Category.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Rule schema version in this particular Cost Category.
        #[builder(into)]
        pub rule_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for the Cost Category rules used to categorize costs. See below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::costexplorer::CostCategoryRule>,
        >,
        /// Configuration block for the split charge rules used to allocate your charges between your Cost Category values. See below.
        #[builder(into, default)]
        pub split_charge_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::costexplorer::CostCategorySplitChargeRule>>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CostCategoryResult {
        /// ARN of the cost category.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Default value for the cost category.
        pub default_value: pulumi_gestalt_rust::Output<Option<String>>,
        /// Effective end data of your Cost Category.
        pub effective_end: pulumi_gestalt_rust::Output<String>,
        /// The Cost Category's effective start date. It can only be a billing start date (first day of the month). If the date isn't provided, it's the first day of the current month. Dates can't be before the previous twelve months, or in the future. For example `2022-11-01T00:00:00Z`.
        ///
        /// The following arguments are optional:
        pub effective_start: pulumi_gestalt_rust::Output<String>,
        /// Unique name for the Cost Category.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Rule schema version in this particular Cost Category.
        pub rule_version: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the Cost Category rules used to categorize costs. See below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::costexplorer::CostCategoryRule>,
        >,
        /// Configuration block for the split charge rules used to allocate your charges between your Cost Category values. See below.
        pub split_charge_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::costexplorer::CostCategorySplitChargeRule>>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CostCategoryArgs,
    ) -> CostCategoryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_value_binding = args.default_value.get_output(context);
        let effective_start_binding = args.effective_start.get_output(context);
        let name_binding = args.name.get_output(context);
        let rule_version_binding = args.rule_version.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let split_charge_rules_binding = args.split_charge_rules.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:costexplorer/costCategory:CostCategory".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultValue".into(),
                    value: &default_value_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "effectiveStart".into(),
                    value: &effective_start_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleVersion".into(),
                    value: &rule_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "splitChargeRules".into(),
                    value: &split_charge_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CostCategoryResult {
            arn: o.get_field("arn"),
            default_value: o.get_field("defaultValue"),
            effective_end: o.get_field("effectiveEnd"),
            effective_start: o.get_field("effectiveStart"),
            name: o.get_field("name"),
            rule_version: o.get_field("ruleVersion"),
            rules: o.get_field("rules"),
            split_charge_rules: o.get_field("splitChargeRules"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}

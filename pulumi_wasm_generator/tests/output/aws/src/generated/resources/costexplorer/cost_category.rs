/// Provides a CE Cost Category.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod cost_category {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CostCategoryArgs {
        /// Default value for the cost category.
        #[builder(into, default)]
        pub default_value: pulumi_wasm_rust::Output<Option<String>>,
        /// The Cost Category's effective start date. It can only be a billing start date (first day of the month). If the date isn't provided, it's the first day of the current month. Dates can't be before the previous twelve months, or in the future. For example `2022-11-01T00:00:00Z`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub effective_start: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique name for the Cost Category.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Rule schema version in this particular Cost Category.
        #[builder(into)]
        pub rule_version: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the Cost Category rules used to categorize costs. See below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::costexplorer::CostCategoryRule>,
        >,
        /// Configuration block for the split charge rules used to allocate your charges between your Cost Category values. See below.
        #[builder(into, default)]
        pub split_charge_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::costexplorer::CostCategorySplitChargeRule>>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CostCategoryResult {
        /// ARN of the cost category.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Default value for the cost category.
        pub default_value: pulumi_wasm_rust::Output<Option<String>>,
        /// Effective end data of your Cost Category.
        pub effective_end: pulumi_wasm_rust::Output<String>,
        /// The Cost Category's effective start date. It can only be a billing start date (first day of the month). If the date isn't provided, it's the first day of the current month. Dates can't be before the previous twelve months, or in the future. For example `2022-11-01T00:00:00Z`.
        ///
        /// The following arguments are optional:
        pub effective_start: pulumi_wasm_rust::Output<String>,
        /// Unique name for the Cost Category.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Rule schema version in this particular Cost Category.
        pub rule_version: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the Cost Category rules used to categorize costs. See below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::costexplorer::CostCategoryRule>,
        >,
        /// Configuration block for the split charge rules used to allocate your charges between your Cost Category values. See below.
        pub split_charge_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::costexplorer::CostCategorySplitChargeRule>>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: CostCategoryArgs) -> CostCategoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_value_binding = args.default_value.get_inner();
        let effective_start_binding = args.effective_start.get_inner();
        let name_binding = args.name.get_inner();
        let rule_version_binding = args.rule_version.get_inner();
        let rules_binding = args.rules.get_inner();
        let split_charge_rules_binding = args.split_charge_rules.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:costexplorer/costCategory:CostCategory".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultValue".into(),
                    value: &default_value_binding,
                },
                register_interface::ObjectField {
                    name: "effectiveStart".into(),
                    value: &effective_start_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ruleVersion".into(),
                    value: &rule_version_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "splitChargeRules".into(),
                    value: &split_charge_rules_binding,
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
                    name: "defaultValue".into(),
                },
                register_interface::ResultField {
                    name: "effectiveEnd".into(),
                },
                register_interface::ResultField {
                    name: "effectiveStart".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ruleVersion".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "splitChargeRules".into(),
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
        CostCategoryResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            default_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultValue").unwrap(),
            ),
            effective_end: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveEnd").unwrap(),
            ),
            effective_start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveStart").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rule_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleVersion").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            split_charge_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("splitChargeRules").unwrap(),
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

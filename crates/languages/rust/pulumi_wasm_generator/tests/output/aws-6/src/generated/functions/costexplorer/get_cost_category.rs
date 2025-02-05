pub mod get_cost_category {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCostCategoryArgs {
        /// Unique name for the Cost Category.
        #[builder(into)]
        pub cost_category_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Configuration block for the specific `Tag` to use for `Expression`. See below.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetCostCategoryResult {
        pub cost_category_arn: pulumi_wasm_rust::Output<String>,
        /// Default value for the cost category.
        pub default_value: pulumi_wasm_rust::Output<String>,
        /// Effective end data of your Cost Category.
        pub effective_end: pulumi_wasm_rust::Output<String>,
        /// Effective state data of your Cost Category.
        pub effective_start: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Rule schema version in this particular Cost Category.
        pub rule_version: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the `Expression` object used to categorize costs. See below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::costexplorer::GetCostCategoryRule>,
        >,
        /// Configuration block for the split charge rules used to allocate your charges between your Cost Category values. See below.
        pub split_charge_rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::costexplorer::GetCostCategorySplitChargeRule>,
        >,
        /// Configuration block for the specific `Tag` to use for `Expression`. See below.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCostCategoryArgs,
    ) -> GetCostCategoryResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cost_category_arn_binding = args
            .cost_category_arn
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:costexplorer/getCostCategory:getCostCategory".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "costCategoryArn".into(),
                    value: &cost_category_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCostCategoryResult {
            cost_category_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("costCategoryArn"),
            ),
            default_value: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultValue"),
            ),
            effective_end: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveEnd"),
            ),
            effective_start: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveStart"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            rule_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ruleVersion"),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
            split_charge_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("splitChargeRules"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}

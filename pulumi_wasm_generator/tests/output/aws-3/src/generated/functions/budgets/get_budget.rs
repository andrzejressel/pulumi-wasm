pub mod get_budget {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBudgetArgs {
        /// The ID of the target account for budget. Will use current user's account_id by default if omitted.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of a budget. Unique within accounts.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The prefix of the name of a budget. Unique within accounts.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetBudgetResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Object containing [AutoAdjustData] which determines the budget amount for an auto-adjusting budget.
        pub auto_adjust_datas: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetAutoAdjustData>,
        >,
        /// Boolean indicating whether this budget has been exceeded.
        pub budget_exceeded: pulumi_wasm_rust::Output<bool>,
        /// The total amount of cost, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage that you want to track with your budget. Contains object Spend.
        pub budget_limits: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetBudgetLimit>,
        >,
        /// Whether this budget tracks monetary cost or usage.
        pub budget_type: pulumi_wasm_rust::Output<String>,
        /// The spend objects that are associated with this budget. The actualSpend tracks how much you've used, cost, usage, RI units, or Savings Plans units and the forecastedSpend tracks how much that you're predicted to spend based on your historical usage profile.
        pub calculated_spends: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetCalculatedSpend>,
        >,
        /// A list of CostFilter name/values pair to apply to budget.
        pub cost_filters: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetCostFilter>,
        >,
        /// Object containing CostTypes The types of cost included in a budget, such as tax and subscriptions.
        pub cost_types: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetCostType>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Object containing Budget Notifications. Can be used multiple times to define more than one budget notification.
        pub notifications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetNotification>,
        >,
        /// Object containing Planned Budget Limits. Can be used multiple times to plan more than one budget limit. See [PlannedBudgetLimits](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_Budget.html#awscostmanagement-Type-budgets_Budget-PlannedBudgetLimits) documentation.
        pub planned_limits: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetPlannedLimit>,
        >,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The end of the time period covered by the budget. There are no restrictions on the end date. Format: `2017-01-01_12:00`.
        pub time_period_end: pulumi_wasm_rust::Output<String>,
        /// The start of the time period covered by the budget. If you don't specify a start date, AWS defaults to the start of your chosen time period. The start date must come before the end date. Format: `2017-01-01_12:00`.
        pub time_period_start: pulumi_wasm_rust::Output<String>,
        /// The length of time until a budget resets the actual and forecasted spend. Valid values: `MONTHLY`, `QUARTERLY`, `ANNUALLY`, and `DAILY`.
        pub time_unit: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetBudgetArgs,
    ) -> GetBudgetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:budgets/getBudget:getBudget".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoAdjustDatas".into(),
                },
                register_interface::ResultField {
                    name: "budgetExceeded".into(),
                },
                register_interface::ResultField {
                    name: "budgetLimits".into(),
                },
                register_interface::ResultField {
                    name: "budgetType".into(),
                },
                register_interface::ResultField {
                    name: "calculatedSpends".into(),
                },
                register_interface::ResultField {
                    name: "costFilters".into(),
                },
                register_interface::ResultField {
                    name: "costTypes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "notifications".into(),
                },
                register_interface::ResultField {
                    name: "plannedLimits".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timePeriodEnd".into(),
                },
                register_interface::ResultField {
                    name: "timePeriodStart".into(),
                },
                register_interface::ResultField {
                    name: "timeUnit".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBudgetResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_adjust_datas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoAdjustDatas").unwrap(),
            ),
            budget_exceeded: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("budgetExceeded").unwrap(),
            ),
            budget_limits: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("budgetLimits").unwrap(),
            ),
            budget_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("budgetType").unwrap(),
            ),
            calculated_spends: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("calculatedSpends").unwrap(),
            ),
            cost_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("costFilters").unwrap(),
            ),
            cost_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("costTypes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            notifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notifications").unwrap(),
            ),
            planned_limits: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plannedLimits").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            time_period_end: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timePeriodEnd").unwrap(),
            ),
            time_period_start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timePeriodStart").unwrap(),
            ),
            time_unit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeUnit").unwrap(),
            ),
        }
    }
}

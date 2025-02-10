#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_budget {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBudgetArgs {
        /// The ID of the target account for budget. Will use current user's account_id by default if omitted.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of a budget. Unique within accounts.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The prefix of the name of a budget. Unique within accounts.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetBudgetResult {
        pub account_id: pulumi_gestalt_rust::Output<String>,
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Object containing [AutoAdjustData] which determines the budget amount for an auto-adjusting budget.
        pub auto_adjust_datas: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetAutoAdjustData>,
        >,
        /// Boolean indicating whether this budget has been exceeded.
        pub budget_exceeded: pulumi_gestalt_rust::Output<bool>,
        /// The total amount of cost, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage that you want to track with your budget. Contains object Spend.
        pub budget_limits: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetBudgetLimit>,
        >,
        /// Whether this budget tracks monetary cost or usage.
        pub budget_type: pulumi_gestalt_rust::Output<String>,
        /// The spend objects that are associated with this budget. The actualSpend tracks how much you've used, cost, usage, RI units, or Savings Plans units and the forecastedSpend tracks how much that you're predicted to spend based on your historical usage profile.
        pub calculated_spends: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetCalculatedSpend>,
        >,
        /// A list of CostFilter name/values pair to apply to budget.
        pub cost_filters: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetCostFilter>,
        >,
        /// Object containing CostTypes The types of cost included in a budget, such as tax and subscriptions.
        pub cost_types: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetCostType>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub name_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// Object containing Budget Notifications. Can be used multiple times to define more than one budget notification.
        pub notifications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetNotification>,
        >,
        /// Object containing Planned Budget Limits. Can be used multiple times to plan more than one budget limit. See [PlannedBudgetLimits](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_Budget.html#awscostmanagement-Type-budgets_Budget-PlannedBudgetLimits) documentation.
        pub planned_limits: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::budgets::GetBudgetPlannedLimit>,
        >,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The end of the time period covered by the budget. There are no restrictions on the end date. Format: `2017-01-01_12:00`.
        pub time_period_end: pulumi_gestalt_rust::Output<String>,
        /// The start of the time period covered by the budget. If you don't specify a start date, AWS defaults to the start of your chosen time period. The start date must come before the end date. Format: `2017-01-01_12:00`.
        pub time_period_start: pulumi_gestalt_rust::Output<String>,
        /// The length of time until a budget resets the actual and forecasted spend. Valid values: `MONTHLY`, `QUARTERLY`, `ANNUALLY`, and `DAILY`.
        pub time_unit: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBudgetArgs,
    ) -> GetBudgetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:budgets/getBudget:getBudget".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBudgetResult {
            account_id: o.get_field("accountId"),
            arn: o.get_field("arn"),
            auto_adjust_datas: o.get_field("autoAdjustDatas"),
            budget_exceeded: o.get_field("budgetExceeded"),
            budget_limits: o.get_field("budgetLimits"),
            budget_type: o.get_field("budgetType"),
            calculated_spends: o.get_field("calculatedSpends"),
            cost_filters: o.get_field("costFilters"),
            cost_types: o.get_field("costTypes"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            notifications: o.get_field("notifications"),
            planned_limits: o.get_field("plannedLimits"),
            tags: o.get_field("tags"),
            time_period_end: o.get_field("timePeriodEnd"),
            time_period_start: o.get_field("timePeriodStart"),
            time_unit: o.get_field("timeUnit"),
        }
    }
}

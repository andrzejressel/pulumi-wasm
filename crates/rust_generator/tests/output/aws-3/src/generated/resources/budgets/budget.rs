/// Provides a budgets budget resource. Budgets use the cost visualization provided by Cost Explorer to show you the status of your budgets, to provide forecasts of your estimated costs, and to track your AWS usage, including your free tier usage.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   ec2:
///     type: aws:budgets:Budget
///     properties:
///       name: budget-ec2-monthly
///       budgetType: COST
///       limitAmount: '1200'
///       limitUnit: USD
///       timePeriodEnd: 2087-06-15_00:00
///       timePeriodStart: 2017-07-01_00:00
///       timeUnit: MONTHLY
///       costFilters:
///         - name: Service
///           values:
///             - Amazon Elastic Compute Cloud - Compute
///       notifications:
///         - comparisonOperator: GREATER_THAN
///           threshold: 100
///           thresholdType: PERCENTAGE
///           notificationType: FORECASTED
///           subscriberEmailAddresses:
///             - test@example.com
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// Create a budget for *$100*.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cost = budget::create(
///         "cost",
///         BudgetArgs::builder()
///             .budget_type("COST")
///             .limit_amount("100")
///             .limit_unit("USD")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Create a budget with planned budget limits.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cost = budget::create(
///         "cost",
///         BudgetArgs::builder()
///             .planned_limits(
///                 vec![
///                     BudgetPlannedLimit::builder().amount("100")
///                     .startTime("2017-07-01_00:00").unit("USD").build_struct(),
///                     BudgetPlannedLimit::builder().amount("200")
///                     .startTime("2017-08-01_00:00").unit("USD").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// Create a budget for s3 with a limit of *3 GB* of storage.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let s3 = budget::create(
///         "s3",
///         BudgetArgs::builder()
///             .budget_type("USAGE")
///             .limit_amount("3")
///             .limit_unit("GB")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Create a Savings Plan Utilization Budget
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let savingsPlanUtilization = budget::create(
///         "savingsPlanUtilization",
///         BudgetArgs::builder()
///             .budget_type("SAVINGS_PLANS_UTILIZATION")
///             .cost_types(
///                 BudgetCostTypes::builder()
///                     .includeCredit(false)
///                     .includeDiscount(false)
///                     .includeOtherSubscription(false)
///                     .includeRecurring(false)
///                     .includeRefund(false)
///                     .includeSubscription(true)
///                     .includeSupport(false)
///                     .includeTax(false)
///                     .includeUpfront(false)
///                     .useBlended(false)
///                     .build_struct(),
///             )
///             .limit_amount("100.0")
///             .limit_unit("PERCENTAGE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Create a RI Utilization Budget
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let riUtilization = budget::create(
///         "riUtilization",
///         BudgetArgs::builder()
///             .budget_type("RI_UTILIZATION")
///             .cost_filters(
///                 vec![
///                     BudgetCostFilter::builder().name("Service")
///                     .values(vec!["Amazon Relational Database Service",]).build_struct(),
///                 ],
///             )
///             .cost_types(
///                 BudgetCostTypes::builder()
///                     .includeCredit(false)
///                     .includeDiscount(false)
///                     .includeOtherSubscription(false)
///                     .includeRecurring(false)
///                     .includeRefund(false)
///                     .includeSubscription(true)
///                     .includeSupport(false)
///                     .includeTax(false)
///                     .includeUpfront(false)
///                     .useBlended(false)
///                     .build_struct(),
///             )
///             .limit_amount("100.0")
///             .limit_unit("PERCENTAGE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Create a Cost Filter using Resource Tags
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cost = budget::create(
///         "cost",
///         BudgetArgs::builder()
///             .cost_filters(
///                 vec![
///                     BudgetCostFilter::builder().name("TagKeyValue")
///                     .values(vec!["TagKey$TagValue",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// Create a cost_filter using resource tags, obtaining the tag value from a variable
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cost = budget::create(
///         "cost",
///         BudgetArgs::builder()
///             .cost_filters(
///                 vec![
///                     BudgetCostFilter::builder().name("TagKeyValue")
///                     .values(vec!["TagKey$${tagValue}",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import budgets using `AccountID:BudgetName`. For example:
///
/// ```sh
/// $ pulumi import aws:budgets/budget:Budget myBudget 123456789012:myBudget
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod budget {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BudgetArgs {
        /// The ID of the target account for budget. Will use current user's account_id by default if omitted.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Object containing AutoAdjustData which determines the budget amount for an auto-adjusting budget.
        #[builder(into, default)]
        pub auto_adjust_data: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::budgets::BudgetAutoAdjustData>,
        >,
        /// Whether this budget tracks monetary cost or usage.
        #[builder(into)]
        pub budget_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of CostFilter name/values pair to apply to budget.
        #[builder(into, default)]
        pub cost_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::budgets::BudgetCostFilter>>,
        >,
        /// Object containing CostTypes The types of cost included in a budget, such as tax and subscriptions.
        #[builder(into, default)]
        pub cost_types: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::budgets::BudgetCostTypes>,
        >,
        /// The amount of cost or usage being measured for a budget.
        #[builder(into, default)]
        pub limit_amount: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The unit of measurement used for the budget forecast, actual spend, or budget threshold, such as dollars or GB. See [Spend](http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/data-type-spend.html) documentation.
        #[builder(into, default)]
        pub limit_unit: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of a budget. Unique within accounts.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The prefix of the name of a budget. Unique within accounts.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Object containing Budget Notifications. Can be used multiple times to define more than one budget notification.
        #[builder(into, default)]
        pub notifications: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::budgets::BudgetNotification>>,
        >,
        /// Object containing Planned Budget Limits. Can be used multiple times to plan more than one budget limit. See [PlannedBudgetLimits](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_Budget.html#awscostmanagement-Type-budgets_Budget-PlannedBudgetLimits) documentation.
        #[builder(into, default)]
        pub planned_limits: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::budgets::BudgetPlannedLimit>>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The end of the time period covered by the budget. There are no restrictions on the end date. Format: `2017-01-01_12:00`.
        #[builder(into, default)]
        pub time_period_end: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The start of the time period covered by the budget. If you don't specify a start date, AWS defaults to the start of your chosen time period. The start date must come before the end date. Format: `2017-01-01_12:00`.
        #[builder(into, default)]
        pub time_period_start: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The length of time until a budget resets the actual and forecasted spend. Valid values: `MONTHLY`, `QUARTERLY`, `ANNUALLY`, and `DAILY`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub time_unit: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BudgetResult {
        /// The ID of the target account for budget. Will use current user's account_id by default if omitted.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the budget.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Object containing AutoAdjustData which determines the budget amount for an auto-adjusting budget.
        pub auto_adjust_data: pulumi_gestalt_rust::Output<
            Option<super::super::types::budgets::BudgetAutoAdjustData>,
        >,
        /// Whether this budget tracks monetary cost or usage.
        pub budget_type: pulumi_gestalt_rust::Output<String>,
        /// A list of CostFilter name/values pair to apply to budget.
        pub cost_filters: pulumi_gestalt_rust::Output<
            Vec<super::super::types::budgets::BudgetCostFilter>,
        >,
        /// Object containing CostTypes The types of cost included in a budget, such as tax and subscriptions.
        pub cost_types: pulumi_gestalt_rust::Output<
            super::super::types::budgets::BudgetCostTypes,
        >,
        /// The amount of cost or usage being measured for a budget.
        pub limit_amount: pulumi_gestalt_rust::Output<String>,
        /// The unit of measurement used for the budget forecast, actual spend, or budget threshold, such as dollars or GB. See [Spend](http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/data-type-spend.html) documentation.
        pub limit_unit: pulumi_gestalt_rust::Output<String>,
        /// The name of a budget. Unique within accounts.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The prefix of the name of a budget. Unique within accounts.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Object containing Budget Notifications. Can be used multiple times to define more than one budget notification.
        pub notifications: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::budgets::BudgetNotification>>,
        >,
        /// Object containing Planned Budget Limits. Can be used multiple times to plan more than one budget limit. See [PlannedBudgetLimits](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_Budget.html#awscostmanagement-Type-budgets_Budget-PlannedBudgetLimits) documentation.
        pub planned_limits: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::budgets::BudgetPlannedLimit>>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The end of the time period covered by the budget. There are no restrictions on the end date. Format: `2017-01-01_12:00`.
        pub time_period_end: pulumi_gestalt_rust::Output<Option<String>>,
        /// The start of the time period covered by the budget. If you don't specify a start date, AWS defaults to the start of your chosen time period. The start date must come before the end date. Format: `2017-01-01_12:00`.
        pub time_period_start: pulumi_gestalt_rust::Output<String>,
        /// The length of time until a budget resets the actual and forecasted spend. Valid values: `MONTHLY`, `QUARTERLY`, `ANNUALLY`, and `DAILY`.
        ///
        /// The following arguments are optional:
        pub time_unit: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BudgetArgs,
    ) -> BudgetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let auto_adjust_data_binding_1 = args.auto_adjust_data.get_output(context);
        let auto_adjust_data_binding = auto_adjust_data_binding_1.get_inner();
        let budget_type_binding_1 = args.budget_type.get_output(context);
        let budget_type_binding = budget_type_binding_1.get_inner();
        let cost_filters_binding_1 = args.cost_filters.get_output(context);
        let cost_filters_binding = cost_filters_binding_1.get_inner();
        let cost_types_binding_1 = args.cost_types.get_output(context);
        let cost_types_binding = cost_types_binding_1.get_inner();
        let limit_amount_binding_1 = args.limit_amount.get_output(context);
        let limit_amount_binding = limit_amount_binding_1.get_inner();
        let limit_unit_binding_1 = args.limit_unit.get_output(context);
        let limit_unit_binding = limit_unit_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let name_prefix_binding_1 = args.name_prefix.get_output(context);
        let name_prefix_binding = name_prefix_binding_1.get_inner();
        let notifications_binding_1 = args.notifications.get_output(context);
        let notifications_binding = notifications_binding_1.get_inner();
        let planned_limits_binding_1 = args.planned_limits.get_output(context);
        let planned_limits_binding = planned_limits_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let time_period_end_binding_1 = args.time_period_end.get_output(context);
        let time_period_end_binding = time_period_end_binding_1.get_inner();
        let time_period_start_binding_1 = args.time_period_start.get_output(context);
        let time_period_start_binding = time_period_start_binding_1.get_inner();
        let time_unit_binding_1 = args.time_unit.get_output(context);
        let time_unit_binding = time_unit_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:budgets/budget:Budget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "autoAdjustData".into(),
                    value: &auto_adjust_data_binding,
                },
                register_interface::ObjectField {
                    name: "budgetType".into(),
                    value: &budget_type_binding,
                },
                register_interface::ObjectField {
                    name: "costFilters".into(),
                    value: &cost_filters_binding,
                },
                register_interface::ObjectField {
                    name: "costTypes".into(),
                    value: &cost_types_binding,
                },
                register_interface::ObjectField {
                    name: "limitAmount".into(),
                    value: &limit_amount_binding,
                },
                register_interface::ObjectField {
                    name: "limitUnit".into(),
                    value: &limit_unit_binding,
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
                    name: "notifications".into(),
                    value: &notifications_binding,
                },
                register_interface::ObjectField {
                    name: "plannedLimits".into(),
                    value: &planned_limits_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timePeriodEnd".into(),
                    value: &time_period_end_binding,
                },
                register_interface::ObjectField {
                    name: "timePeriodStart".into(),
                    value: &time_period_start_binding,
                },
                register_interface::ObjectField {
                    name: "timeUnit".into(),
                    value: &time_unit_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BudgetResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auto_adjust_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoAdjustData"),
            ),
            budget_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("budgetType"),
            ),
            cost_filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("costFilters"),
            ),
            cost_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("costTypes"),
            ),
            limit_amount: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("limitAmount"),
            ),
            limit_unit: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("limitUnit"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            notifications: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notifications"),
            ),
            planned_limits: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("plannedLimits"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            time_period_end: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timePeriodEnd"),
            ),
            time_period_start: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timePeriodStart"),
            ),
            time_unit: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeUnit"),
            ),
        }
    }
}

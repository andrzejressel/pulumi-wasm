/// Budget configuration for a billing account.
///
///
/// To get more information about Budget, see:
///
/// * [API documentation](https://cloud.google.com/billing/docs/reference/budget/rest/v1/billingAccounts.budgets)
/// * How-to Guides
///     * [Creating a budget](https://cloud.google.com/billing/docs/how-to/budgets)
///
/// > **Warning:** If you are using User ADCs (Application Default Credentials) with this resource,
/// you must specify a `billing_project` and set `user_project_override` to true
/// in the provider configuration. Otherwise the Billing Budgets API will return a 403 error.
/// Your account must have the `serviceusage.services.use` permission on the
/// `billing_project` you defined.
///
/// ## Example Usage
///
/// ### Billing Budget Basic
///
///
/// ```yaml
/// resources:
///   budget:
///     type: gcp:billing:Budget
///     properties:
///       billingAccount: ${account.id}
///       displayName: Example Billing Budget
///       amount:
///         specifiedAmount:
///           currencyCode: USD
///           units: '100000'
///       thresholdRules:
///         - thresholdPercent: 0.5
/// variables:
///   account:
///     fn::invoke:
///       function: gcp:organizations:getBillingAccount
///       arguments:
///         billingAccount: 000000-0000000-0000000-000000
/// ```
/// ### Billing Budget Lastperiod
///
///
/// ```yaml
/// resources:
///   budget:
///     type: gcp:billing:Budget
///     properties:
///       billingAccount: ${account.id}
///       displayName: Example Billing Budget
///       budgetFilter:
///         projects:
///           - projects/${project.number}
///       amount:
///         lastPeriodAmount: true
///       thresholdRules:
///         - thresholdPercent: 10
/// variables:
///   account:
///     fn::invoke:
///       function: gcp:organizations:getBillingAccount
///       arguments:
///         billingAccount: 000000-0000000-0000000-000000
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Billing Budget Filter
///
///
/// ```yaml
/// resources:
///   budget:
///     type: gcp:billing:Budget
///     properties:
///       billingAccount: ${account.id}
///       displayName: Example Billing Budget
///       budgetFilter:
///         projects:
///           - projects/${project.number}
///         creditTypesTreatment: INCLUDE_SPECIFIED_CREDITS
///         services:
///           - services/24E6-581D-38E5
///         creditTypes:
///           - PROMOTION
///           - FREE_TIER
///         resourceAncestors:
///           - organizations/123456789
///       amount:
///         specifiedAmount:
///           currencyCode: USD
///           units: '100000'
///       thresholdRules:
///         - thresholdPercent: 0.5
///         - thresholdPercent: 0.9
///           spendBasis: FORECASTED_SPEND
/// variables:
///   account:
///     fn::invoke:
///       function: gcp:organizations:getBillingAccount
///       arguments:
///         billingAccount: 000000-0000000-0000000-000000
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Billing Budget Notify
///
///
/// ```yaml
/// resources:
///   budget:
///     type: gcp:billing:Budget
///     properties:
///       billingAccount: ${account.id}
///       displayName: Example Billing Budget
///       budgetFilter:
///         projects:
///           - projects/${project.number}
///       amount:
///         specifiedAmount:
///           currencyCode: USD
///           units: '100000'
///       thresholdRules:
///         - thresholdPercent: 1
///         - thresholdPercent: 1
///           spendBasis: FORECASTED_SPEND
///       allUpdatesRule:
///         monitoringNotificationChannels:
///           - ${notificationChannel.id}
///         disableDefaultIamRecipients: true
///   notificationChannel:
///     type: gcp:monitoring:NotificationChannel
///     name: notification_channel
///     properties:
///       displayName: Example Notification Channel
///       type: email
///       labels:
///         email_address: address@example.com
/// variables:
///   account:
///     fn::invoke:
///       function: gcp:organizations:getBillingAccount
///       arguments:
///         billingAccount: 000000-0000000-0000000-000000
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Billing Budget Notify Project Recipient
///
///
/// ```yaml
/// resources:
///   budget:
///     type: gcp:billing:Budget
///     properties:
///       billingAccount: ${account.id}
///       displayName: Example Billing Budget
///       budgetFilter:
///         projects:
///           - projects/${project.number}
///       amount:
///         specifiedAmount:
///           currencyCode: USD
///           units: '100000'
///       allUpdatesRule:
///         monitoringNotificationChannels: []
///         enableProjectLevelRecipients: true
/// variables:
///   account:
///     fn::invoke:
///       function: gcp:organizations:getBillingAccount
///       arguments:
///         billingAccount: 000000-0000000-0000000-000000
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Billing Budget Customperiod
///
///
/// ```yaml
/// resources:
///   budget:
///     type: gcp:billing:Budget
///     properties:
///       billingAccount: ${account.id}
///       displayName: Example Billing Budget
///       budgetFilter:
///         projects:
///           - projects/${project.number}
///         creditTypesTreatment: EXCLUDE_ALL_CREDITS
///         services:
///           - services/24E6-581D-38E5
///         customPeriod:
///           startDate:
///             year: 2022
///             month: 1
///             day: 1
///           endDate:
///             year: 2023
///             month: 12
///             day: 31
///       amount:
///         specifiedAmount:
///           currencyCode: USD
///           units: '100000'
///       thresholdRules:
///         - thresholdPercent: 0.5
///         - thresholdPercent: 0.9
/// variables:
///   account:
///     fn::invoke:
///       function: gcp:organizations:getBillingAccount
///       arguments:
///         billingAccount: 000000-0000000-0000000-000000
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Budget can be imported using any of these accepted formats:
///
/// * `billingAccounts/{{billing_account}}/budgets/{{name}}`
///
/// * `{{billing_account}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Budget can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:billing/budget:Budget default billingAccounts/{{billing_account}}/budgets/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:billing/budget:Budget default {{billing_account}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:billing/budget:Budget default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod budget {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BudgetArgs {
        /// Defines notifications that are sent on every update to the billing account's spend, regardless of the thresholds defined
        /// using threshold rules.
        #[builder(into, default)]
        pub all_updates_rule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::billing::BudgetAllUpdatesRule>,
        >,
        /// The budgeted amount for each usage period.
        /// Structure is documented below.
        #[builder(into)]
        pub amount: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::billing::BudgetAmount,
        >,
        /// ID of the billing account to set a budget on.
        #[builder(into)]
        pub billing_account: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Filters that define which resources are used to compute the actual spend against the budget.
        #[builder(into, default)]
        pub budget_filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::billing::BudgetBudgetFilter>,
        >,
        /// User data for display name in UI. Must be <= 60 chars.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ownership scope of the budget. The ownership scope and users' IAM permissions determine who has full access to the
        /// budget's data. Possible values: ["OWNERSHIP_SCOPE_UNSPECIFIED", "ALL_USERS", "BILLING_ACCOUNT"]
        #[builder(into, default)]
        pub ownership_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Rules that trigger alerts (notifications of thresholds being crossed) when spend exceeds the specified percentages of
        /// the budget.
        #[builder(into, default)]
        pub threshold_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::billing::BudgetThresholdRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BudgetResult {
        /// Defines notifications that are sent on every update to the billing account's spend, regardless of the thresholds defined
        /// using threshold rules.
        pub all_updates_rule: pulumi_gestalt_rust::Output<
            Option<super::super::types::billing::BudgetAllUpdatesRule>,
        >,
        /// The budgeted amount for each usage period.
        /// Structure is documented below.
        pub amount: pulumi_gestalt_rust::Output<
            super::super::types::billing::BudgetAmount,
        >,
        /// ID of the billing account to set a budget on.
        pub billing_account: pulumi_gestalt_rust::Output<String>,
        /// Filters that define which resources are used to compute the actual spend against the budget.
        pub budget_filter: pulumi_gestalt_rust::Output<
            super::super::types::billing::BudgetBudgetFilter,
        >,
        /// User data for display name in UI. Must be <= 60 chars.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Resource name of the budget. The resource name
        /// implies the scope of a budget. Values are of the form
        /// billingAccounts/{billingAccountId}/budgets/{budgetId}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ownership scope of the budget. The ownership scope and users' IAM permissions determine who has full access to the
        /// budget's data. Possible values: ["OWNERSHIP_SCOPE_UNSPECIFIED", "ALL_USERS", "BILLING_ACCOUNT"]
        pub ownership_scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// Rules that trigger alerts (notifications of thresholds being crossed) when spend exceeds the specified percentages of
        /// the budget.
        pub threshold_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::billing::BudgetThresholdRule>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BudgetArgs,
    ) -> BudgetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let all_updates_rule_binding = args.all_updates_rule.get_output(context);
        let amount_binding = args.amount.get_output(context);
        let billing_account_binding = args.billing_account.get_output(context);
        let budget_filter_binding = args.budget_filter.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let ownership_scope_binding = args.ownership_scope.get_output(context);
        let threshold_rules_binding = args.threshold_rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:billing/budget:Budget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allUpdatesRule".into(),
                    value: &all_updates_rule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "amount".into(),
                    value: &amount_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingAccount".into(),
                    value: &billing_account_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "budgetFilter".into(),
                    value: &budget_filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownershipScope".into(),
                    value: &ownership_scope_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thresholdRules".into(),
                    value: &threshold_rules_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BudgetResult {
            all_updates_rule: o.get_field("allUpdatesRule"),
            amount: o.get_field("amount"),
            billing_account: o.get_field("billingAccount"),
            budget_filter: o.get_field("budgetFilter"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            ownership_scope: o.get_field("ownershipScope"),
            threshold_rules: o.get_field("thresholdRules"),
        }
    }
}

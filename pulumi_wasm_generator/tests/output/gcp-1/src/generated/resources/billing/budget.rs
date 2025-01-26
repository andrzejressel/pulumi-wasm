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
pub mod budget {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BudgetArgs {
        /// Defines notifications that are sent on every update to the billing account's spend, regardless of the thresholds defined
        /// using threshold rules.
        #[builder(into, default)]
        pub all_updates_rule: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::billing::BudgetAllUpdatesRule>,
        >,
        /// The budgeted amount for each usage period.
        /// Structure is documented below.
        #[builder(into)]
        pub amount: pulumi_wasm_rust::InputOrOutput<
            super::super::types::billing::BudgetAmount,
        >,
        /// ID of the billing account to set a budget on.
        #[builder(into)]
        pub billing_account: pulumi_wasm_rust::InputOrOutput<String>,
        /// Filters that define which resources are used to compute the actual spend against the budget.
        #[builder(into, default)]
        pub budget_filter: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::billing::BudgetBudgetFilter>,
        >,
        /// User data for display name in UI. Must be <= 60 chars.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ownership scope of the budget. The ownership scope and users' IAM permissions determine who has full access to the
        /// budget's data. Possible values: ["OWNERSHIP_SCOPE_UNSPECIFIED", "ALL_USERS", "BILLING_ACCOUNT"]
        #[builder(into, default)]
        pub ownership_scope: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Rules that trigger alerts (notifications of thresholds being crossed) when spend exceeds the specified percentages of
        /// the budget.
        #[builder(into, default)]
        pub threshold_rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::billing::BudgetThresholdRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BudgetResult {
        /// Defines notifications that are sent on every update to the billing account's spend, regardless of the thresholds defined
        /// using threshold rules.
        pub all_updates_rule: pulumi_wasm_rust::Output<
            Option<super::super::types::billing::BudgetAllUpdatesRule>,
        >,
        /// The budgeted amount for each usage period.
        /// Structure is documented below.
        pub amount: pulumi_wasm_rust::Output<super::super::types::billing::BudgetAmount>,
        /// ID of the billing account to set a budget on.
        pub billing_account: pulumi_wasm_rust::Output<String>,
        /// Filters that define which resources are used to compute the actual spend against the budget.
        pub budget_filter: pulumi_wasm_rust::Output<
            super::super::types::billing::BudgetBudgetFilter,
        >,
        /// User data for display name in UI. Must be <= 60 chars.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource name of the budget. The resource name
        /// implies the scope of a budget. Values are of the form
        /// billingAccounts/{billingAccountId}/budgets/{budgetId}.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ownership scope of the budget. The ownership scope and users' IAM permissions determine who has full access to the
        /// budget's data. Possible values: ["OWNERSHIP_SCOPE_UNSPECIFIED", "ALL_USERS", "BILLING_ACCOUNT"]
        pub ownership_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Rules that trigger alerts (notifications of thresholds being crossed) when spend exceeds the specified percentages of
        /// the budget.
        pub threshold_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::billing::BudgetThresholdRule>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BudgetArgs,
    ) -> BudgetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let all_updates_rule_binding = args
            .all_updates_rule
            .get_output(context)
            .get_inner();
        let amount_binding = args.amount.get_output(context).get_inner();
        let billing_account_binding = args
            .billing_account
            .get_output(context)
            .get_inner();
        let budget_filter_binding = args.budget_filter.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let ownership_scope_binding = args
            .ownership_scope
            .get_output(context)
            .get_inner();
        let threshold_rules_binding = args
            .threshold_rules
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:billing/budget:Budget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allUpdatesRule".into(),
                    value: &all_updates_rule_binding,
                },
                register_interface::ObjectField {
                    name: "amount".into(),
                    value: &amount_binding,
                },
                register_interface::ObjectField {
                    name: "billingAccount".into(),
                    value: &billing_account_binding,
                },
                register_interface::ObjectField {
                    name: "budgetFilter".into(),
                    value: &budget_filter_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "ownershipScope".into(),
                    value: &ownership_scope_binding,
                },
                register_interface::ObjectField {
                    name: "thresholdRules".into(),
                    value: &threshold_rules_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BudgetResult {
            all_updates_rule: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allUpdatesRule"),
            ),
            amount: pulumi_wasm_rust::__private::into_domain(o.extract_field("amount")),
            billing_account: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("billingAccount"),
            ),
            budget_filter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("budgetFilter"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            ownership_scope: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownershipScope"),
            ),
            threshold_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("thresholdRules"),
            ),
        }
    }
}

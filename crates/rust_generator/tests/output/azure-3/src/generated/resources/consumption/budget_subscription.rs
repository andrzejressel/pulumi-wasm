/// Manages a Subscription Consumption Budget.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: eastus
///   exampleActionGroup:
///     type: azure:monitoring:ActionGroup
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       shortName: example
///   exampleBudgetSubscription:
///     type: azure:consumption:BudgetSubscription
///     name: example
///     properties:
///       name: example
///       subscriptionId: ${current.id}
///       amount: 1000
///       timeGrain: Monthly
///       timePeriod:
///         startDate: 2022-06-01T00:00:00Z
///         endDate: 2022-07-01T00:00:00Z
///       filter:
///         dimensions:
///           - name: ResourceGroupName
///             values:
///               - ${example.name}
///         tags:
///           - name: foo
///             values:
///               - bar
///               - baz
///       notifications:
///         - enabled: true
///           threshold: 90
///           operator: EqualTo
///           contactEmails:
///             - foo@example.com
///             - bar@example.com
///           contactGroups:
///             - ${exampleActionGroup.id}
///           contactRoles:
///             - Owner
///         - enabled: false
///           threshold: 100
///           operator: GreaterThan
///           thresholdType: Forecasted
///           contactEmails:
///             - foo@example.com
///             - bar@example.com
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Subscription Consumption Budgets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:consumption/budgetSubscription:BudgetSubscription example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Consumption/budgets/subscription1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod budget_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BudgetSubscriptionArgs {
        /// The total amount of cost to track with the budget.
        #[builder(into)]
        pub amount: pulumi_gestalt_rust::InputOrOutput<f64>,
        /// (Optional) The ETag of the Subscription Consumption Budget.
        #[builder(into, default)]
        pub etag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `filter` block as defined below.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::consumption::BudgetSubscriptionFilter>,
        >,
        /// The name which should be used for this Subscription Consumption Budget. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `notification` blocks as defined below.
        #[builder(into)]
        pub notifications: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::consumption::BudgetSubscriptionNotification>,
        >,
        /// The ID of the Subscription for which to create a Consumption Budget. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `subscription_id` property can accept a subscription ID e.g. `00000000-0000-0000-0000-000000000000` or the subscription resource ID e.g. `/subscriptions/00000000-0000-0000-0000-000000000000`. In version 3.0 this property will only accept the subscription resource ID.
        #[builder(into)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time covered by a budget. Tracking of the amount will be reset based on the time grain. Must be one of `BillingAnnual`, `BillingMonth`, `BillingQuarter`, `Annually`, `Monthly` and `Quarterly`. Defaults to `Monthly`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub time_grain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `time_period` block as defined below.
        #[builder(into)]
        pub time_period: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::consumption::BudgetSubscriptionTimePeriod,
        >,
    }
    #[allow(dead_code)]
    pub struct BudgetSubscriptionResult {
        /// The total amount of cost to track with the budget.
        pub amount: pulumi_gestalt_rust::Output<f64>,
        /// (Optional) The ETag of the Subscription Consumption Budget.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// A `filter` block as defined below.
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::consumption::BudgetSubscriptionFilter>,
        >,
        /// The name which should be used for this Subscription Consumption Budget. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `notification` blocks as defined below.
        pub notifications: pulumi_gestalt_rust::Output<
            Vec<super::super::types::consumption::BudgetSubscriptionNotification>,
        >,
        /// The ID of the Subscription for which to create a Consumption Budget. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `subscription_id` property can accept a subscription ID e.g. `00000000-0000-0000-0000-000000000000` or the subscription resource ID e.g. `/subscriptions/00000000-0000-0000-0000-000000000000`. In version 3.0 this property will only accept the subscription resource ID.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
        /// The time covered by a budget. Tracking of the amount will be reset based on the time grain. Must be one of `BillingAnnual`, `BillingMonth`, `BillingQuarter`, `Annually`, `Monthly` and `Quarterly`. Defaults to `Monthly`. Changing this forces a new resource to be created.
        pub time_grain: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `time_period` block as defined below.
        pub time_period: pulumi_gestalt_rust::Output<
            super::super::types::consumption::BudgetSubscriptionTimePeriod,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BudgetSubscriptionArgs,
    ) -> BudgetSubscriptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let amount_binding = args.amount.get_output(context).get_inner();
        let etag_binding = args.etag.get_output(context).get_inner();
        let filter_binding = args.filter.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let notifications_binding = args.notifications.get_output(context).get_inner();
        let subscription_id_binding = args
            .subscription_id
            .get_output(context)
            .get_inner();
        let time_grain_binding = args.time_grain.get_output(context).get_inner();
        let time_period_binding = args.time_period.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:consumption/budgetSubscription:BudgetSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "amount".into(),
                    value: &amount_binding,
                },
                register_interface::ObjectField {
                    name: "etag".into(),
                    value: &etag_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notifications".into(),
                    value: &notifications_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
                register_interface::ObjectField {
                    name: "timeGrain".into(),
                    value: &time_grain_binding,
                },
                register_interface::ObjectField {
                    name: "timePeriod".into(),
                    value: &time_period_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BudgetSubscriptionResult {
            amount: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("amount"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            notifications: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notifications"),
            ),
            subscription_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionId"),
            ),
            time_grain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeGrain"),
            ),
            time_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timePeriod"),
            ),
        }
    }
}

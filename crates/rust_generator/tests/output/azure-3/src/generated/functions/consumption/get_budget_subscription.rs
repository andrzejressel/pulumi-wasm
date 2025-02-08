#[allow(clippy::doc_lazy_continuation)]
pub mod get_budget_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBudgetSubscriptionArgs {
        /// The name of this Consumption Budget.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the subscription.
        #[builder(into)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBudgetSubscriptionResult {
        /// The total amount of cost to track with the budget.
        pub amount: pulumi_gestalt_rust::Output<f64>,
        /// A `filter` block as defined below.
        pub filters: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::consumption::GetBudgetSubscriptionFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the tag to use for the filter.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `notification` block as defined below.
        pub notifications: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::consumption::GetBudgetSubscriptionNotification,
            >,
        >,
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
        /// The time covered by a budget.
        pub time_grain: pulumi_gestalt_rust::Output<String>,
        /// A `time_period` block as defined below.
        pub time_periods: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::consumption::GetBudgetSubscriptionTimePeriod>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetBudgetSubscriptionArgs,
    ) -> GetBudgetSubscriptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let subscription_id_binding = args
            .subscription_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:consumption/getBudgetSubscription:getBudgetSubscription"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBudgetSubscriptionResult {
            amount: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("amount"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
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
            time_periods: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timePeriods"),
            ),
        }
    }
}

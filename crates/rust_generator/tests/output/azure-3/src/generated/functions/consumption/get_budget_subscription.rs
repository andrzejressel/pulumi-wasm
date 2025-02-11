#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetBudgetSubscriptionArgs,
    ) -> GetBudgetSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let subscription_id_binding = args.subscription_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:consumption/getBudgetSubscription:getBudgetSubscription"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBudgetSubscriptionResult {
            amount: o.get_field("amount"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            notifications: o.get_field("notifications"),
            subscription_id: o.get_field("subscriptionId"),
            time_grain: o.get_field("timeGrain"),
            time_periods: o.get_field("timePeriods"),
        }
    }
}

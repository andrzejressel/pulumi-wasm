#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_budget_resource_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBudgetResourceGroupArgs {
        /// The name of this Consumption Budget.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the subscription.
        #[builder(into)]
        pub resource_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBudgetResourceGroupResult {
        /// The total amount of cost to track with the budget.
        pub amount: pulumi_gestalt_rust::Output<f64>,
        /// A `filter` block as defined below.
        pub filters: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::consumption::GetBudgetResourceGroupFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the tag used for the filter.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `notification` block as defined below.
        pub notifications: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::consumption::GetBudgetResourceGroupNotification,
            >,
        >,
        pub resource_group_id: pulumi_gestalt_rust::Output<String>,
        /// The time covered by a budget.
        pub time_grain: pulumi_gestalt_rust::Output<String>,
        /// A `time_period` block as defined below.
        pub time_periods: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::consumption::GetBudgetResourceGroupTimePeriod,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBudgetResourceGroupArgs,
    ) -> GetBudgetResourceGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_id_binding = args.resource_group_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:consumption/getBudgetResourceGroup:getBudgetResourceGroup"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupId".into(),
                    value: &resource_group_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBudgetResourceGroupResult {
            amount: o.get_field("amount"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            notifications: o.get_field("notifications"),
            resource_group_id: o.get_field("resourceGroupId"),
            time_grain: o.get_field("timeGrain"),
            time_periods: o.get_field("timePeriods"),
        }
    }
}

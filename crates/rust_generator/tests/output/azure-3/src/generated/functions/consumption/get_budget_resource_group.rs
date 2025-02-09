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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetBudgetResourceGroupArgs,
    ) -> GetBudgetResourceGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_id_binding_1 = args.resource_group_id.get_output(context);
        let resource_group_id_binding = resource_group_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:consumption/getBudgetResourceGroup:getBudgetResourceGroup"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupId".into(),
                    value: &resource_group_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBudgetResourceGroupResult {
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
            resource_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupId"),
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

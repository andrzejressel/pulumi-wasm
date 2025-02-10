/// Manages a Resource Group Consumption Budget.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder().location("eastus").name("example").build_struct(),
///     );
///     let exampleActionGroup = action_group::create(
///         "exampleActionGroup",
///         ActionGroupArgs::builder()
///             .name("example")
///             .resource_group_name("${example.name}")
///             .short_name("example")
///             .build_struct(),
///     );
///     let exampleBudgetResourceGroup = budget_resource_group::create(
///         "exampleBudgetResourceGroup",
///         BudgetResourceGroupArgs::builder()
///             .amount(1000)
///             .filter(
///                 BudgetResourceGroupFilter::builder()
///                     .dimensions(
///                         vec![
///                             BudgetResourceGroupFilterDimension::builder()
///                             .name("ResourceId").values(vec!["${exampleActionGroup.id}",])
///                             .build_struct(),
///                         ],
///                     )
///                     .tags(
///                         vec![
///                             BudgetResourceGroupFilterTag::builder().name("foo")
///                             .values(vec!["bar", "baz",]).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .notifications(
///                 vec![
///                     BudgetResourceGroupNotification::builder()
///                     .contactEmails(vec!["foo@example.com", "bar@example.com",])
///                     .contactGroups(vec!["${exampleActionGroup.id}",])
///                     .contactRoles(vec!["Owner",]).enabled(true).operator("EqualTo")
///                     .threshold(90).thresholdType("Forecasted").build_struct(),
///                     BudgetResourceGroupNotification::builder()
///                     .contactEmails(vec!["foo@example.com", "bar@example.com",])
///                     .enabled(false).operator("GreaterThan").threshold(100)
///                     .build_struct(),
///                 ],
///             )
///             .resource_group_id("${example.id}")
///             .time_grain("Monthly")
///             .time_period(
///                 BudgetResourceGroupTimePeriod::builder()
///                     .endDate("2022-07-01T00:00:00Z")
///                     .startDate("2022-06-01T00:00:00Z")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Resource Group Consumption Budgets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:consumption/budgetResourceGroup:BudgetResourceGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Consumption/budgets/resourceGroup1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod budget_resource_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BudgetResourceGroupArgs {
        /// The total amount of cost to track with the budget.
        #[builder(into)]
        pub amount: pulumi_gestalt_rust::InputOrOutput<f64>,
        /// (Optional) The ETag of the Resource Group Consumption Budget
        #[builder(into, default)]
        pub etag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `filter` block as defined below.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::consumption::BudgetResourceGroupFilter>,
        >,
        /// The name which should be used for this Resource Group Consumption Budget. Changing this forces a new Resource Group Consumption Budget to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `notification` blocks as defined below.
        #[builder(into)]
        pub notifications: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::consumption::BudgetResourceGroupNotification>,
        >,
        /// The ID of the Resource Group to create the consumption budget for in the form of /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1. Changing this forces a new Resource Group Consumption Budget to be created.
        #[builder(into)]
        pub resource_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time covered by a budget. Tracking of the amount will be reset based on the time grain. Must be one of `BillingAnnual`, `BillingMonth`, `BillingQuarter`, `Annually`, `Monthly` and `Quarterly`. Defaults to `Monthly`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub time_grain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `time_period` block as defined below.
        #[builder(into)]
        pub time_period: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::consumption::BudgetResourceGroupTimePeriod,
        >,
    }
    #[allow(dead_code)]
    pub struct BudgetResourceGroupResult {
        /// The total amount of cost to track with the budget.
        pub amount: pulumi_gestalt_rust::Output<f64>,
        /// (Optional) The ETag of the Resource Group Consumption Budget
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// A `filter` block as defined below.
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::consumption::BudgetResourceGroupFilter>,
        >,
        /// The name which should be used for this Resource Group Consumption Budget. Changing this forces a new Resource Group Consumption Budget to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `notification` blocks as defined below.
        pub notifications: pulumi_gestalt_rust::Output<
            Vec<super::super::types::consumption::BudgetResourceGroupNotification>,
        >,
        /// The ID of the Resource Group to create the consumption budget for in the form of /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1. Changing this forces a new Resource Group Consumption Budget to be created.
        pub resource_group_id: pulumi_gestalt_rust::Output<String>,
        /// The time covered by a budget. Tracking of the amount will be reset based on the time grain. Must be one of `BillingAnnual`, `BillingMonth`, `BillingQuarter`, `Annually`, `Monthly` and `Quarterly`. Defaults to `Monthly`. Changing this forces a new resource to be created.
        pub time_grain: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `time_period` block as defined below.
        pub time_period: pulumi_gestalt_rust::Output<
            super::super::types::consumption::BudgetResourceGroupTimePeriod,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BudgetResourceGroupArgs,
    ) -> BudgetResourceGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let amount_binding = args.amount.get_output(context);
        let etag_binding = args.etag.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let name_binding = args.name.get_output(context);
        let notifications_binding = args.notifications.get_output(context);
        let resource_group_id_binding = args.resource_group_id.get_output(context);
        let time_grain_binding = args.time_grain.get_output(context);
        let time_period_binding = args.time_period.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:consumption/budgetResourceGroup:BudgetResourceGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "amount".into(),
                    value: amount_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "etag".into(),
                    value: etag_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notifications".into(),
                    value: notifications_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupId".into(),
                    value: resource_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeGrain".into(),
                    value: time_grain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timePeriod".into(),
                    value: time_period_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BudgetResourceGroupResult {
            amount: o.get_field("amount"),
            etag: o.get_field("etag"),
            filter: o.get_field("filter"),
            name: o.get_field("name"),
            notifications: o.get_field("notifications"),
            resource_group_id: o.get_field("resourceGroupId"),
            time_grain: o.get_field("timeGrain"),
            time_period: o.get_field("timePeriod"),
        }
    }
}

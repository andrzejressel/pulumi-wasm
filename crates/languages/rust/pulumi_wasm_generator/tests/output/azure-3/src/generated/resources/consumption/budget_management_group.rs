/// Manages a Consumption Budget for a Management Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = group::create(
///         "example",
///         GroupArgs::builder().display_name("example").build_struct(),
///     );
///     let exampleBudgetManagementGroup = budget_management_group::create(
///         "exampleBudgetManagementGroup",
///         BudgetManagementGroupArgs::builder()
///             .amount(1000)
///             .filter(
///                 BudgetManagementGroupFilter::builder()
///                     .dimensions(
///                         vec![
///                             BudgetManagementGroupFilterDimension::builder()
///                             .name("ResourceGroupName")
///                             .values(vec!["${exampleResourceGroup.name}",])
///                             .build_struct(),
///                         ],
///                     )
///                     .tags(
///                         vec![
///                             BudgetManagementGroupFilterTag::builder().name("foo")
///                             .values(vec!["bar", "baz",]).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .management_group_id("${example.id}")
///             .name("example")
///             .notifications(
///                 vec![
///                     BudgetManagementGroupNotification::builder()
///                     .contactEmails(vec!["foo@example.com", "bar@example.com",])
///                     .enabled(true).operator("EqualTo").threshold(90).build_struct(),
///                     BudgetManagementGroupNotification::builder()
///                     .contactEmails(vec!["foo@example.com", "bar@example.com",])
///                     .enabled(false).operator("GreaterThan").threshold(100)
///                     .thresholdType("Forecasted").build_struct(),
///                 ],
///             )
///             .time_grain("Monthly")
///             .time_period(
///                 BudgetManagementGroupTimePeriod::builder()
///                     .endDate("2022-07-01T00:00:00Z")
///                     .startDate("2022-06-01T00:00:00Z")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleResourceGroup = resource_group::create(
///         "exampleResourceGroup",
///         ResourceGroupArgs::builder().location("eastus").name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Management Group Consumption Budgets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:consumption/budgetManagementGroup:BudgetManagementGroup example /providers/Microsoft.Management/managementGroups/00000000-0000-0000-0000-000000000000/providers/Microsoft.Consumption/budgets/budget1
/// ```
///
pub mod budget_management_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BudgetManagementGroupArgs {
        /// The total amount of cost to track with the budget.
        #[builder(into)]
        pub amount: pulumi_wasm_rust::InputOrOutput<f64>,
        /// (Optional) The ETag of the Management Group Consumption Budget.
        #[builder(into, default)]
        pub etag: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `filter` block as defined below.
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::consumption::BudgetManagementGroupFilter>,
        >,
        /// The ID of the Management Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub management_group_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Management Group Consumption Budget. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `notification` blocks as defined below.
        #[builder(into)]
        pub notifications: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::consumption::BudgetManagementGroupNotification>,
        >,
        /// The time covered by a budget. Tracking of the amount will be reset based on the time grain. Must be one of `BillingAnnual`, `BillingMonth`, `BillingQuarter`, `Annually`, `Monthly` and `Quarterly`. Defaults to `Monthly`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub time_grain: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `time_period` block as defined below.
        #[builder(into)]
        pub time_period: pulumi_wasm_rust::InputOrOutput<
            super::super::types::consumption::BudgetManagementGroupTimePeriod,
        >,
    }
    #[allow(dead_code)]
    pub struct BudgetManagementGroupResult {
        /// The total amount of cost to track with the budget.
        pub amount: pulumi_wasm_rust::Output<f64>,
        /// (Optional) The ETag of the Management Group Consumption Budget.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// A `filter` block as defined below.
        pub filter: pulumi_wasm_rust::Output<
            Option<super::super::types::consumption::BudgetManagementGroupFilter>,
        >,
        /// The ID of the Management Group. Changing this forces a new resource to be created.
        pub management_group_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Management Group Consumption Budget. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `notification` blocks as defined below.
        pub notifications: pulumi_wasm_rust::Output<
            Vec<super::super::types::consumption::BudgetManagementGroupNotification>,
        >,
        /// The time covered by a budget. Tracking of the amount will be reset based on the time grain. Must be one of `BillingAnnual`, `BillingMonth`, `BillingQuarter`, `Annually`, `Monthly` and `Quarterly`. Defaults to `Monthly`. Changing this forces a new resource to be created.
        pub time_grain: pulumi_wasm_rust::Output<Option<String>>,
        /// A `time_period` block as defined below.
        pub time_period: pulumi_wasm_rust::Output<
            super::super::types::consumption::BudgetManagementGroupTimePeriod,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BudgetManagementGroupArgs,
    ) -> BudgetManagementGroupResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let amount_binding = args.amount.get_output(context).get_inner();
        let etag_binding = args.etag.get_output(context).get_inner();
        let filter_binding = args.filter.get_output(context).get_inner();
        let management_group_id_binding = args
            .management_group_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let notifications_binding = args.notifications.get_output(context).get_inner();
        let time_grain_binding = args.time_grain.get_output(context).get_inner();
        let time_period_binding = args.time_period.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:consumption/budgetManagementGroup:BudgetManagementGroup"
                .into(),
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
                    name: "managementGroupId".into(),
                    value: &management_group_id_binding,
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
        BudgetManagementGroupResult {
            amount: pulumi_wasm_rust::__private::into_domain(o.extract_field("amount")),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            filter: pulumi_wasm_rust::__private::into_domain(o.extract_field("filter")),
            management_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managementGroupId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            notifications: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notifications"),
            ),
            time_grain: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeGrain"),
            ),
            time_period: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timePeriod"),
            ),
        }
    }
}

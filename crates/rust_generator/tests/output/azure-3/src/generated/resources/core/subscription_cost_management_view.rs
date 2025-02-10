/// Manages an Azure Cost Management View for a Subscription.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subscription_cost_management_view::create(
///         "example",
///         SubscriptionCostManagementViewArgs::builder()
///             .accumulated(false)
///             .chart_type("StackedColumn")
///             .dataset(
///                 SubscriptionCostManagementViewDataset::builder()
///                     .aggregations(
///                         vec![
///                             SubscriptionCostManagementViewDatasetAggregation::builder()
///                             .columnName("Cost").name("totalCost").build_struct(),
///                         ],
///                     )
///                     .granularity("Monthly")
///                     .build_struct(),
///             )
///             .display_name("Cost View per Month")
///             .name("example")
///             .report_type("Usage")
///             .subscription_id("/subscription/00000000-0000-0000-0000-000000000000")
///             .timeframe("MonthToDate")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Cost Management View for a Subscriptions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/subscriptionCostManagementView:SubscriptionCostManagementView example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.CostManagement/views/costmanagementview
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subscription_cost_management_view {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionCostManagementViewArgs {
        /// Whether the costs data in the Cost Management View are accumulated over time. Changing this forces a new Cost Management View for a Subscription to be created.
        #[builder(into)]
        pub accumulated: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Chart type of the main view in Cost Analysis. Possible values are `Area`, `GroupedColumn`, `Line`, `StackedColumn` and `Table`.
        #[builder(into)]
        pub chart_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `dataset` block as defined below.
        #[builder(into)]
        pub dataset: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::core::SubscriptionCostManagementViewDataset,
        >,
        /// User visible input name of the Cost Management View.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `kpi` blocks as defined below, to show in Cost Analysis UI.
        #[builder(into, default)]
        pub kpis: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::core::SubscriptionCostManagementViewKpi>>,
        >,
        /// The name which should be used for this Cost Management View for a Subscription. Changing this forces a new Cost Management View for a Subscription to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `pivot` blocks as defined below, containing the configuration of 3 sub-views in the Cost Analysis UI. Non table views should have three pivots.
        #[builder(into, default)]
        pub pivots: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::core::SubscriptionCostManagementViewPivot>>,
        >,
        /// The type of the report. The only possible value is `Usage`.
        #[builder(into)]
        pub report_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subscription this View is scoped to. Changing this forces a new Cost Management View for a Subscription to be created.
        #[builder(into)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time frame for pulling data for the report. Possible values are `Custom`, `MonthToDate`, `WeekToDate` and `YearToDate`.
        #[builder(into)]
        pub timeframe: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionCostManagementViewResult {
        /// Whether the costs data in the Cost Management View are accumulated over time. Changing this forces a new Cost Management View for a Subscription to be created.
        pub accumulated: pulumi_gestalt_rust::Output<bool>,
        /// Chart type of the main view in Cost Analysis. Possible values are `Area`, `GroupedColumn`, `Line`, `StackedColumn` and `Table`.
        pub chart_type: pulumi_gestalt_rust::Output<String>,
        /// A `dataset` block as defined below.
        pub dataset: pulumi_gestalt_rust::Output<
            super::super::types::core::SubscriptionCostManagementViewDataset,
        >,
        /// User visible input name of the Cost Management View.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `kpi` blocks as defined below, to show in Cost Analysis UI.
        pub kpis: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::core::SubscriptionCostManagementViewKpi>>,
        >,
        /// The name which should be used for this Cost Management View for a Subscription. Changing this forces a new Cost Management View for a Subscription to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `pivot` blocks as defined below, containing the configuration of 3 sub-views in the Cost Analysis UI. Non table views should have three pivots.
        pub pivots: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::core::SubscriptionCostManagementViewPivot>>,
        >,
        /// The type of the report. The only possible value is `Usage`.
        pub report_type: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subscription this View is scoped to. Changing this forces a new Cost Management View for a Subscription to be created.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
        /// The time frame for pulling data for the report. Possible values are `Custom`, `MonthToDate`, `WeekToDate` and `YearToDate`.
        pub timeframe: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubscriptionCostManagementViewArgs,
    ) -> SubscriptionCostManagementViewResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accumulated_binding = args.accumulated.get_output(context);
        let chart_type_binding = args.chart_type.get_output(context);
        let dataset_binding = args.dataset.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let kpis_binding = args.kpis.get_output(context);
        let name_binding = args.name.get_output(context);
        let pivots_binding = args.pivots.get_output(context);
        let report_type_binding = args.report_type.get_output(context);
        let subscription_id_binding = args.subscription_id.get_output(context);
        let timeframe_binding = args.timeframe.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:core/subscriptionCostManagementView:SubscriptionCostManagementView"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accumulated".into(),
                    value: accumulated_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "chartType".into(),
                    value: chart_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataset".into(),
                    value: dataset_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kpis".into(),
                    value: kpis_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pivots".into(),
                    value: pivots_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reportType".into(),
                    value: report_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionId".into(),
                    value: subscription_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeframe".into(),
                    value: timeframe_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubscriptionCostManagementViewResult {
            accumulated: o.get_field("accumulated"),
            chart_type: o.get_field("chartType"),
            dataset: o.get_field("dataset"),
            display_name: o.get_field("displayName"),
            kpis: o.get_field("kpis"),
            name: o.get_field("name"),
            pivots: o.get_field("pivots"),
            report_type: o.get_field("reportType"),
            subscription_id: o.get_field("subscriptionId"),
            timeframe: o.get_field("timeframe"),
        }
    }
}

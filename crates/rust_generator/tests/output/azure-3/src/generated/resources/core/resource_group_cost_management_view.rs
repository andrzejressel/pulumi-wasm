/// Manages an Azure Cost Management View for a Resource Group.
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
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleResourceGroupCostManagementView = resource_group_cost_management_view::create(
///         "exampleResourceGroupCostManagementView",
///         ResourceGroupCostManagementViewArgs::builder()
///             .accumulated(false)
///             .chart_type("StackedColumn")
///             .dataset(
///                 ResourceGroupCostManagementViewDataset::builder()
///                     .aggregations(
///                         vec![
///                             ResourceGroupCostManagementViewDatasetAggregation::builder()
///                             .columnName("Cost").name("totalCost").build_struct(),
///                         ],
///                     )
///                     .granularity("Monthly")
///                     .build_struct(),
///             )
///             .display_name("Cost View per Month")
///             .name("example")
///             .report_type("Usage")
///             .resource_group_id("${example.id}")
///             .timeframe("MonthToDate")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Cost Management View for a Resource Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/resourceGroupCostManagementView:ResourceGroupCostManagementView example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.CostManagement/views/costmanagementview
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_group_cost_management_view {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceGroupCostManagementViewArgs {
        /// Whether the costs data in the Cost Management View are accumulated over time. Changing this forces a new Cost Management View for a Resource Group to be created.
        #[builder(into)]
        pub accumulated: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Chart type of the main view in Cost Analysis. Possible values are `Area`, `GroupedColumn`, `Line`, `StackedColumn` and `Table`.
        #[builder(into)]
        pub chart_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `dataset` block as defined below.
        #[builder(into)]
        pub dataset: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::core::ResourceGroupCostManagementViewDataset,
        >,
        /// User visible input name of the Cost Management View.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `kpi` blocks as defined below, to show in Cost Analysis UI.
        #[builder(into, default)]
        pub kpis: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::core::ResourceGroupCostManagementViewKpi>>,
        >,
        /// The name which should be used for this Cost Management View for a Resource Group. Changing this forces a new Cost Management View for a Resource Group to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `pivot` blocks as defined below, containing the configuration of 3 sub-views in the Cost Analysis UI. Non table views should have three pivots.
        #[builder(into, default)]
        pub pivots: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::core::ResourceGroupCostManagementViewPivot>>,
        >,
        /// The type of the report. The only possible value is `Usage`.
        #[builder(into)]
        pub report_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Resource Group this View is scoped to. Changing this forces a new Cost Management View for a Resource Group to be created.
        #[builder(into)]
        pub resource_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time frame for pulling data for the report. Possible values are `Custom`, `MonthToDate`, `WeekToDate` and `YearToDate`.
        #[builder(into)]
        pub timeframe: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceGroupCostManagementViewResult {
        /// Whether the costs data in the Cost Management View are accumulated over time. Changing this forces a new Cost Management View for a Resource Group to be created.
        pub accumulated: pulumi_gestalt_rust::Output<bool>,
        /// Chart type of the main view in Cost Analysis. Possible values are `Area`, `GroupedColumn`, `Line`, `StackedColumn` and `Table`.
        pub chart_type: pulumi_gestalt_rust::Output<String>,
        /// A `dataset` block as defined below.
        pub dataset: pulumi_gestalt_rust::Output<
            super::super::types::core::ResourceGroupCostManagementViewDataset,
        >,
        /// User visible input name of the Cost Management View.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `kpi` blocks as defined below, to show in Cost Analysis UI.
        pub kpis: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::core::ResourceGroupCostManagementViewKpi>>,
        >,
        /// The name which should be used for this Cost Management View for a Resource Group. Changing this forces a new Cost Management View for a Resource Group to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `pivot` blocks as defined below, containing the configuration of 3 sub-views in the Cost Analysis UI. Non table views should have three pivots.
        pub pivots: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::core::ResourceGroupCostManagementViewPivot>>,
        >,
        /// The type of the report. The only possible value is `Usage`.
        pub report_type: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Resource Group this View is scoped to. Changing this forces a new Cost Management View for a Resource Group to be created.
        pub resource_group_id: pulumi_gestalt_rust::Output<String>,
        /// The time frame for pulling data for the report. Possible values are `Custom`, `MonthToDate`, `WeekToDate` and `YearToDate`.
        pub timeframe: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResourceGroupCostManagementViewArgs,
    ) -> ResourceGroupCostManagementViewResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accumulated_binding = args.accumulated.get_output(context).get_inner();
        let chart_type_binding = args.chart_type.get_output(context).get_inner();
        let dataset_binding = args.dataset.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let kpis_binding = args.kpis.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let pivots_binding = args.pivots.get_output(context).get_inner();
        let report_type_binding = args.report_type.get_output(context).get_inner();
        let resource_group_id_binding = args
            .resource_group_id
            .get_output(context)
            .get_inner();
        let timeframe_binding = args.timeframe.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/resourceGroupCostManagementView:ResourceGroupCostManagementView"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accumulated".into(),
                    value: &accumulated_binding,
                },
                register_interface::ObjectField {
                    name: "chartType".into(),
                    value: &chart_type_binding,
                },
                register_interface::ObjectField {
                    name: "dataset".into(),
                    value: &dataset_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "kpis".into(),
                    value: &kpis_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pivots".into(),
                    value: &pivots_binding,
                },
                register_interface::ObjectField {
                    name: "reportType".into(),
                    value: &report_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupId".into(),
                    value: &resource_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "timeframe".into(),
                    value: &timeframe_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourceGroupCostManagementViewResult {
            accumulated: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accumulated"),
            ),
            chart_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("chartType"),
            ),
            dataset: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataset"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            kpis: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kpis")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            pivots: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pivots"),
            ),
            report_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reportType"),
            ),
            resource_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupId"),
            ),
            timeframe: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeframe"),
            ),
        }
    }
}

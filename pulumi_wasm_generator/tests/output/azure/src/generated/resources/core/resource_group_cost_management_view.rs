/// Manages an Azure Cost Management View for a Resource Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod resource_group_cost_management_view {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceGroupCostManagementViewArgs {
        /// Whether the costs data in the Cost Management View are accumulated over time. Changing this forces a new Cost Management View for a Resource Group to be created.
        #[builder(into)]
        pub accumulated: pulumi_wasm_rust::Output<bool>,
        /// Chart type of the main view in Cost Analysis. Possible values are `Area`, `GroupedColumn`, `Line`, `StackedColumn` and `Table`.
        #[builder(into)]
        pub chart_type: pulumi_wasm_rust::Output<String>,
        /// A `dataset` block as defined below.
        #[builder(into)]
        pub dataset: pulumi_wasm_rust::Output<
            super::super::types::core::ResourceGroupCostManagementViewDataset,
        >,
        /// User visible input name of the Cost Management View.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// One or more `kpi` blocks as defined below, to show in Cost Analysis UI.
        #[builder(into, default)]
        pub kpis: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::core::ResourceGroupCostManagementViewKpi>>,
        >,
        /// The name which should be used for this Cost Management View for a Resource Group. Changing this forces a new Cost Management View for a Resource Group to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `pivot` blocks as defined below, containing the configuration of 3 sub-views in the Cost Analysis UI. Non table views should have three pivots.
        #[builder(into, default)]
        pub pivots: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::core::ResourceGroupCostManagementViewPivot>>,
        >,
        /// The type of the report. The only possible value is `Usage`.
        #[builder(into)]
        pub report_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the Resource Group this View is scoped to. Changing this forces a new Cost Management View for a Resource Group to be created.
        #[builder(into)]
        pub resource_group_id: pulumi_wasm_rust::Output<String>,
        /// The time frame for pulling data for the report. Possible values are `Custom`, `MonthToDate`, `WeekToDate` and `YearToDate`.
        #[builder(into)]
        pub timeframe: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceGroupCostManagementViewResult {
        /// Whether the costs data in the Cost Management View are accumulated over time. Changing this forces a new Cost Management View for a Resource Group to be created.
        pub accumulated: pulumi_wasm_rust::Output<bool>,
        /// Chart type of the main view in Cost Analysis. Possible values are `Area`, `GroupedColumn`, `Line`, `StackedColumn` and `Table`.
        pub chart_type: pulumi_wasm_rust::Output<String>,
        /// A `dataset` block as defined below.
        pub dataset: pulumi_wasm_rust::Output<
            super::super::types::core::ResourceGroupCostManagementViewDataset,
        >,
        /// User visible input name of the Cost Management View.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// One or more `kpi` blocks as defined below, to show in Cost Analysis UI.
        pub kpis: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::core::ResourceGroupCostManagementViewKpi>>,
        >,
        /// The name which should be used for this Cost Management View for a Resource Group. Changing this forces a new Cost Management View for a Resource Group to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `pivot` blocks as defined below, containing the configuration of 3 sub-views in the Cost Analysis UI. Non table views should have three pivots.
        pub pivots: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::core::ResourceGroupCostManagementViewPivot>>,
        >,
        /// The type of the report. The only possible value is `Usage`.
        pub report_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the Resource Group this View is scoped to. Changing this forces a new Cost Management View for a Resource Group to be created.
        pub resource_group_id: pulumi_wasm_rust::Output<String>,
        /// The time frame for pulling data for the report. Possible values are `Custom`, `MonthToDate`, `WeekToDate` and `YearToDate`.
        pub timeframe: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ResourceGroupCostManagementViewArgs,
    ) -> ResourceGroupCostManagementViewResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accumulated_binding = args.accumulated.get_inner();
        let chart_type_binding = args.chart_type.get_inner();
        let dataset_binding = args.dataset.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let kpis_binding = args.kpis.get_inner();
        let name_binding = args.name.get_inner();
        let pivots_binding = args.pivots.get_inner();
        let report_type_binding = args.report_type.get_inner();
        let resource_group_id_binding = args.resource_group_id.get_inner();
        let timeframe_binding = args.timeframe.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/resourceGroupCostManagementView:ResourceGroupCostManagementView"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accumulated".into(),
                },
                register_interface::ResultField {
                    name: "chartType".into(),
                },
                register_interface::ResultField {
                    name: "dataset".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "kpis".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pivots".into(),
                },
                register_interface::ResultField {
                    name: "reportType".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupId".into(),
                },
                register_interface::ResultField {
                    name: "timeframe".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceGroupCostManagementViewResult {
            accumulated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accumulated").unwrap(),
            ),
            chart_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("chartType").unwrap(),
            ),
            dataset: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataset").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            kpis: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kpis").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pivots: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pivots").unwrap(),
            ),
            report_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reportType").unwrap(),
            ),
            resource_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupId").unwrap(),
            ),
            timeframe: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeframe").unwrap(),
            ),
        }
    }
}
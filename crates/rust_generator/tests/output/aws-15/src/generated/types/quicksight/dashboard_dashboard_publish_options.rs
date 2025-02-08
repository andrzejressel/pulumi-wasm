#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DashboardDashboardPublishOptions {
    /// Ad hoc (one-time) filtering option. See ad_hoc_filtering_option.
    #[builder(into, default)]
    #[serde(rename = "adHocFilteringOption")]
    pub r#ad_hoc_filtering_option: Box<Option<super::super::types::quicksight::DashboardDashboardPublishOptionsAdHocFilteringOption>>,
    /// The drill-down options of data points in a dashboard. See data_point_drill_up_down_option.
    #[builder(into, default)]
    #[serde(rename = "dataPointDrillUpDownOption")]
    pub r#data_point_drill_up_down_option: Box<Option<super::super::types::quicksight::DashboardDashboardPublishOptionsDataPointDrillUpDownOption>>,
    /// The data point menu label options of a dashboard. See data_point_menu_label_option.
    #[builder(into, default)]
    #[serde(rename = "dataPointMenuLabelOption")]
    pub r#data_point_menu_label_option: Box<Option<super::super::types::quicksight::DashboardDashboardPublishOptionsDataPointMenuLabelOption>>,
    /// The data point tool tip options of a dashboard. See data_point_tooltip_option.
    #[builder(into, default)]
    #[serde(rename = "dataPointTooltipOption")]
    pub r#data_point_tooltip_option: Box<Option<super::super::types::quicksight::DashboardDashboardPublishOptionsDataPointTooltipOption>>,
    /// Export to .csv option. See export_to_csv_option.
    #[builder(into, default)]
    #[serde(rename = "exportToCsvOption")]
    pub r#export_to_csv_option: Box<Option<super::super::types::quicksight::DashboardDashboardPublishOptionsExportToCsvOption>>,
    /// Determines if hidden fields are exported with a dashboard. See export_with_hidden_fields_option.
    #[builder(into, default)]
    #[serde(rename = "exportWithHiddenFieldsOption")]
    pub r#export_with_hidden_fields_option: Box<Option<super::super::types::quicksight::DashboardDashboardPublishOptionsExportWithHiddenFieldsOption>>,
    /// Sheet controls option. See sheet_controls_option.
    #[builder(into, default)]
    #[serde(rename = "sheetControlsOption")]
    pub r#sheet_controls_option: Box<Option<super::super::types::quicksight::DashboardDashboardPublishOptionsSheetControlsOption>>,
    /// The sheet layout maximization options of a dashboard. See sheet_layout_element_maximization_option.
    #[builder(into, default)]
    #[serde(rename = "sheetLayoutElementMaximizationOption")]
    pub r#sheet_layout_element_maximization_option: Box<Option<super::super::types::quicksight::DashboardDashboardPublishOptionsSheetLayoutElementMaximizationOption>>,
    /// The axis sort options of a dashboard. See visual_axis_sort_option.
    #[builder(into, default)]
    #[serde(rename = "visualAxisSortOption")]
    pub r#visual_axis_sort_option: Box<Option<super::super::types::quicksight::DashboardDashboardPublishOptionsVisualAxisSortOption>>,
    /// The menu options of a visual in a dashboard. See visual_menu_option.
    #[builder(into, default)]
    #[serde(rename = "visualMenuOption")]
    pub r#visual_menu_option: Box<Option<super::super::types::quicksight::DashboardDashboardPublishOptionsVisualMenuOption>>,
}

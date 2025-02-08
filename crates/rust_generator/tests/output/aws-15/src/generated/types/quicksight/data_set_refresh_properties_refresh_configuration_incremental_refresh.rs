#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataSetRefreshPropertiesRefreshConfigurationIncrementalRefresh {
    /// The lookback window setup for an incremental refresh configuration. See lookback_window.
    #[builder(into)]
    #[serde(rename = "lookbackWindow")]
    pub r#lookback_window: Box<super::super::types::quicksight::DataSetRefreshPropertiesRefreshConfigurationIncrementalRefreshLookbackWindow>,
}

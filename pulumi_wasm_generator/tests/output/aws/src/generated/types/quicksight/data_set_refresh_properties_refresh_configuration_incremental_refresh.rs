#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetRefreshPropertiesRefreshConfigurationIncrementalRefresh {
    /// The lookback window setup for an incremental refresh configuration. See lookback_window.
    #[builder(into)]
    #[serde(rename = "lookbackWindow")]
    pub r#lookback_window: Box<super::super::types::quicksight::DataSetRefreshPropertiesRefreshConfigurationIncrementalRefreshLookbackWindow>,
}
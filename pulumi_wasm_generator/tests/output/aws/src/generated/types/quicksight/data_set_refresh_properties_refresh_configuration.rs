#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetRefreshPropertiesRefreshConfiguration {
    /// The incremental refresh for the data set. See incremental_refresh.
    #[builder(into)]
    #[serde(rename = "incrementalRefresh")]
    pub r#incremental_refresh: Box<super::super::types::quicksight::DataSetRefreshPropertiesRefreshConfigurationIncrementalRefresh>,
}

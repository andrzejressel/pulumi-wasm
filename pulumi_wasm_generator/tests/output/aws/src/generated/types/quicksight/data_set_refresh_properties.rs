#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetRefreshProperties {
    /// The refresh configuration for the data set. See refresh_configuration.
    #[builder(into)]
    #[serde(rename = "refreshConfiguration")]
    pub r#refresh_configuration: Box<super::super::types::quicksight::DataSetRefreshPropertiesRefreshConfiguration>,
}
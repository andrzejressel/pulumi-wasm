#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceParametersAwsIotAnalytics {
    /// The name of the data set to which to connect.
    #[builder(into)]
    #[serde(rename = "dataSetName")]
    pub r#data_set_name: Box<String>,
}

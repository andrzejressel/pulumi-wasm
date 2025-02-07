#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesSapoDataParallelismConfig {
    /// he maximum number of records that Amazon AppFlow receives in each page of the response from your SAP application.
    #[builder(into)]
    #[serde(rename = "maxPageSize")]
    pub r#max_page_size: Box<i32>,
}

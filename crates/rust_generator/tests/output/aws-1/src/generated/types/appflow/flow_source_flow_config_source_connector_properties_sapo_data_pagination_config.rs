#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesSapoDataPaginationConfig {
    /// he maximum number of records that Amazon AppFlow receives in each page of the response from your SAP application.
    #[builder(into)]
    #[serde(rename = "maxPageSize")]
    pub r#max_page_size: Box<i32>,
}

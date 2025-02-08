#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceParametersTwitter {
    /// The maximum number of rows to query.
    #[builder(into)]
    #[serde(rename = "maxRows")]
    pub r#max_rows: Box<i32>,
    /// The Twitter query to retrieve the data.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: Box<String>,
}

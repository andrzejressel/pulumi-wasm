#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetManagementServerManagementUri {
    /// The management console api endpoint.
    #[builder(into)]
    #[serde(rename = "api")]
    pub r#api: Box<String>,
    /// The management console webUi.
    #[builder(into)]
    #[serde(rename = "webUi")]
    pub r#web_ui: Box<String>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagementServerManagementUri {
    /// (Output)
    /// The management console api endpoint.
    #[builder(into, default)]
    #[serde(rename = "api")]
    pub r#api: Box<Option<String>>,
    /// (Output)
    /// The management console webUi.
    #[builder(into, default)]
    #[serde(rename = "webUi")]
    pub r#web_ui: Box<Option<String>>,
}

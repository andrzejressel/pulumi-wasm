#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessHomeDirectoryMapping {
    /// Represents an entry and a target.
    #[builder(into)]
    #[serde(rename = "entry")]
    pub r#entry: Box<String>,
    /// Represents the map target.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
}

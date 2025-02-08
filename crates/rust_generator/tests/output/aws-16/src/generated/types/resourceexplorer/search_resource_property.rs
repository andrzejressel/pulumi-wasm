#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SearchResourceProperty {
    /// Details about this property. The content of this field is a JSON object that varies based on the resource type.
    #[builder(into)]
    #[serde(rename = "data")]
    pub r#data: Box<String>,
    /// The date and time that the information about this resource property was last updated.
    #[builder(into)]
    #[serde(rename = "lastReportedAt")]
    pub r#last_reported_at: Box<String>,
    /// Name of this property of the resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

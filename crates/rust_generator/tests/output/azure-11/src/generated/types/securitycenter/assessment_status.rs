#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AssessmentStatus {
    /// Specifies the cause of the assessment status.
    #[builder(into, default)]
    #[serde(rename = "cause")]
    pub r#cause: Box<Option<String>>,
    /// Specifies the programmatic code of the assessment status. Possible values are `Healthy`, `Unhealthy` and `NotApplicable`.
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Box<String>,
    /// Specifies the human readable description of the assessment status.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AnalyzerConfigurationUnusedAccess {
    /// The specified access age in days for which to generate findings for unused access.
    #[builder(into, default)]
    #[serde(rename = "unusedAccessAge")]
    pub r#unused_access_age: Box<Option<i32>>,
}

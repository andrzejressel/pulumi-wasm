#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesetSourceFile {
    /// Textual Content.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// Fingerprint (e.g. github sha) associated with the `File`.
    #[builder(into, default)]
    #[serde(rename = "fingerprint")]
    pub r#fingerprint: Box<Option<String>>,
    /// File name.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

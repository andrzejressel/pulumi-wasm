#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NamespaceState {
    /// (Output)
    /// Code describes the state of a Namespace resource.
    #[builder(into, default)]
    #[serde(rename = "code")]
    pub r#code: Box<Option<String>>,
}

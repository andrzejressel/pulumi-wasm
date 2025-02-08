#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResourceServerScope {
    /// The scope description.
    #[builder(into)]
    #[serde(rename = "scopeDescription")]
    pub r#scope_description: Box<String>,
    /// The scope name.
    #[builder(into)]
    #[serde(rename = "scopeName")]
    pub r#scope_name: Box<String>,
}

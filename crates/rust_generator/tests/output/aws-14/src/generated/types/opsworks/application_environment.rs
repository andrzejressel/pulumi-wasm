#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApplicationEnvironment {
    /// Variable name.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Set visibility of the variable value to `true` or `false`.
    #[builder(into, default)]
    #[serde(rename = "secure")]
    pub r#secure: Box<Option<bool>>,
    /// Variable value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

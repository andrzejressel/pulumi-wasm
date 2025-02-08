#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RunBookDraftParameter {
    /// Specifies the default value of the parameter.
    #[builder(into, default)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Box<Option<String>>,
    /// The name of the parameter.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Whether this parameter is mandatory.
    #[builder(into, default)]
    #[serde(rename = "mandatory")]
    pub r#mandatory: Box<Option<bool>>,
    /// Specifies the position of the parameter.
    #[builder(into, default)]
    #[serde(rename = "position")]
    pub r#position: Box<Option<i32>>,
    /// Specifies the type of this parameter.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}

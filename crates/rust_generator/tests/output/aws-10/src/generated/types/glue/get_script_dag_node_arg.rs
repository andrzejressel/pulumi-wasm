#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetScriptDagNodeArg {
    /// Name of the argument or property.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Boolean if the value is used as a parameter. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "param")]
    pub r#param: Box<Option<bool>>,
    /// Value of the argument or property.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

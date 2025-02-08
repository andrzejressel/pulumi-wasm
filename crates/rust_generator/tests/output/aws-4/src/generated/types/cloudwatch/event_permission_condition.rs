#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EventPermissionCondition {
    /// Key for the condition. Valid values: `aws:PrincipalOrgID`.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Type of condition. Value values: `StringEquals`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Value for the key.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

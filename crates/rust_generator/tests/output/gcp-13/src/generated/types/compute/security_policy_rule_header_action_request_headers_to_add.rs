#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SecurityPolicyRuleHeaderActionRequestHeadersToAdd {
    /// The name of the header to set.
    #[builder(into, default)]
    #[serde(rename = "headerName")]
    pub r#header_name: Box<Option<String>>,
    /// The value to set the named header to.
    #[builder(into, default)]
    #[serde(rename = "headerValue")]
    pub r#header_value: Box<Option<String>>,
}

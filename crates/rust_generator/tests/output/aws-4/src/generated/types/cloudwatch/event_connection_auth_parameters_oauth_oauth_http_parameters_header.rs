#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventConnectionAuthParametersOauthOauthHttpParametersHeader {
    /// Specified whether the value is secret.
    #[builder(into, default)]
    #[serde(rename = "isValueSecret")]
    pub r#is_value_secret: Box<Option<bool>>,
    /// The key for the parameter.
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// The value associated with the key. Created and stored in AWS Secrets Manager if is secret.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

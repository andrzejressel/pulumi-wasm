#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointDeliveryRuleUrlRewriteAction {
    /// This value must start with a `/` and can't be longer than 260 characters.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<String>,
    /// Whether preserve an unmatched path. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "preserveUnmatchedPath")]
    pub r#preserve_unmatched_path: Box<Option<bool>>,
    /// This value must start with a `/` and can't be longer than 260 characters.
    #[builder(into)]
    #[serde(rename = "sourcePattern")]
    pub r#source_pattern: Box<String>,
}

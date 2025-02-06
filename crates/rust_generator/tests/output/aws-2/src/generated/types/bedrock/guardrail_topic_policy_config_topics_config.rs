#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuardrailTopicPolicyConfigTopicsConfig {
    /// Definition of topic in topic policy.
    #[builder(into)]
    #[serde(rename = "definition")]
    pub r#definition: Box<String>,
    /// List of text examples.
    #[builder(into, default)]
    #[serde(rename = "examples")]
    pub r#examples: Box<Option<Vec<String>>>,
    /// Name of topic in topic policy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Type of topic in a policy.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}

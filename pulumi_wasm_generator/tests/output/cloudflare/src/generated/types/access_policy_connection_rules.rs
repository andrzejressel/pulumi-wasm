#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccessPolicyConnectionRules {
    /// The SSH-specific rules that define how users may connect to the targets secured by your application.
    #[builder(into)]
    #[serde(rename = "ssh")]
    pub r#ssh: Box<super::types::AccessPolicyConnectionRulesSsh>,
}

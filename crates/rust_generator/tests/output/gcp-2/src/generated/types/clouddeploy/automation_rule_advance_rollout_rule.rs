#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AutomationRuleAdvanceRolloutRule {
    /// Required. ID of the rule. This id must be unique in the `Automation` resource to which this rule belongs. The format is `a-z{0,62}`.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Optional. Proceeds only after phase name matched any one in the list. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^a-z?$`.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "sourcePhases")]
    pub r#source_phases: Box<Option<Vec<String>>>,
    /// Optional. How long to wait after a rollout is finished.
    #[builder(into, default)]
    #[serde(rename = "wait")]
    pub r#wait: Box<Option<String>>,
}

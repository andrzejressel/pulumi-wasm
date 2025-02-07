#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyPolicySettingsLogScrubbingRule {
    /// Describes if the managed rule is in enabled state or disabled state. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[builder(into)]
    #[serde(rename = "matchVariable")]
    pub r#match_variable: Box<String>,
    /// When matchVariable is a collection, operator used to specify which elements in the collection this rule applies to.
    #[builder(into, default)]
    #[serde(rename = "selector")]
    pub r#selector: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "selectorMatchOperator")]
    pub r#selector_match_operator: Box<Option<String>>,
}

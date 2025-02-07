#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthzPolicyHttpRule {
    /// Describes properties of one or more sources of a request.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "from")]
    pub r#from: Box<Option<super::super::types::networksecurity::AuthzPolicyHttpRuleFrom>>,
    /// Describes properties of one or more targets of a request
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "to")]
    pub r#to: Box<Option<super::super::types::networksecurity::AuthzPolicyHttpRuleTo>>,
    /// CEL expression that describes the conditions to be satisfied for the action. The result of the CEL expression is ANDed with the from and to. Refer to the CEL language reference for a list of available attributes.
    #[builder(into, default)]
    #[serde(rename = "when")]
    pub r#when: Box<Option<String>>,
}

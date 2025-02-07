#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessLevelsAccessLevelBasic {
    /// How the conditions list should be combined to determine if a request
    /// is granted this AccessLevel. If AND is used, each Condition in
    /// conditions must be satisfied for the AccessLevel to be applied. If
    /// OR is used, at least one Condition in conditions must be satisfied
    /// for the AccessLevel to be applied.
    /// Default value is `AND`.
    /// Possible values are: `AND`, `OR`.
    #[builder(into, default)]
    #[serde(rename = "combiningFunction")]
    pub r#combining_function: Box<Option<String>>,
    /// A set of requirements for the AccessLevel to be granted.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Vec<super::super::types::accesscontextmanager::AccessLevelsAccessLevelBasicCondition>>,
}

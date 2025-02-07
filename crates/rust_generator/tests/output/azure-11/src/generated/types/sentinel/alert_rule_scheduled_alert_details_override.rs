#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertRuleScheduledAlertDetailsOverride {
    /// The format containing columns name(s) to override the description of this Sentinel Alert Rule.
    #[builder(into, default)]
    #[serde(rename = "descriptionFormat")]
    pub r#description_format: Box<Option<String>>,
    /// The format containing columns name(s) to override the name of this Sentinel Alert Rule.
    #[builder(into, default)]
    #[serde(rename = "displayNameFormat")]
    pub r#display_name_format: Box<Option<String>>,
    /// A list of `dynamic_property` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "dynamicProperties")]
    pub r#dynamic_properties: Box<Option<Vec<super::super::types::sentinel::AlertRuleScheduledAlertDetailsOverrideDynamicProperty>>>,
    /// The column name to take the alert severity from.
    #[builder(into, default)]
    #[serde(rename = "severityColumnName")]
    pub r#severity_column_name: Box<Option<String>>,
    /// The column name to take the alert tactics from.
    #[builder(into, default)]
    #[serde(rename = "tacticsColumnName")]
    pub r#tactics_column_name: Box<Option<String>>,
}

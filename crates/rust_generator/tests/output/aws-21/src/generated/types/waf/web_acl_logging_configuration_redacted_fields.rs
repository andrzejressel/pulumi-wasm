#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclLoggingConfigurationRedactedFields {
    /// Set of configuration blocks for fields to redact. Detailed below.
    #[builder(into)]
    #[serde(rename = "fieldToMatches")]
    pub r#field_to_matches: Box<Vec<super::super::types::waf::WebAclLoggingConfigurationRedactedFieldsFieldToMatch>>,
}

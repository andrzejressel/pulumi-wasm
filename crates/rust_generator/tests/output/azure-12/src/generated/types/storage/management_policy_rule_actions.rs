#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagementPolicyRuleActions {
    /// A `base_blob` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "baseBlob")]
    pub r#base_blob: Box<Option<super::super::types::storage::ManagementPolicyRuleActionsBaseBlob>>,
    /// A `snapshot` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "snapshot")]
    pub r#snapshot: Box<Option<super::super::types::storage::ManagementPolicyRuleActionsSnapshot>>,
    /// A `version` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<super::super::types::storage::ManagementPolicyRuleActionsVersion>>,
}

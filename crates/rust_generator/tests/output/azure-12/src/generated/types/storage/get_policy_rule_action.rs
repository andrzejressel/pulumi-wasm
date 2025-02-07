#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPolicyRuleAction {
    /// A `base_blob` block as documented below.
    #[builder(into)]
    #[serde(rename = "baseBlobs")]
    pub r#base_blobs: Box<Vec<super::super::types::storage::GetPolicyRuleActionBaseBlob>>,
    /// A `snapshot` block as documented below.
    #[builder(into)]
    #[serde(rename = "snapshots")]
    pub r#snapshots: Box<Vec<super::super::types::storage::GetPolicyRuleActionSnapshot>>,
    /// A `version` block as documented below.
    #[builder(into)]
    #[serde(rename = "versions")]
    pub r#versions: Box<Vec<super::super::types::storage::GetPolicyRuleActionVersion>>,
}

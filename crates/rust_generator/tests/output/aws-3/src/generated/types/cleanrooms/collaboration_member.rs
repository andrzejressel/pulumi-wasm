#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CollaborationMember {
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "memberAbilities")]
    pub r#member_abilities: Box<Vec<String>>,
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

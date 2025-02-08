#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGroupTransitiveMembershipsMembershipRole {
    /// The name of the TransitiveMembershipRole. Possible values: ["OWNER", "MANAGER", "MEMBER"]
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: Box<String>,
}

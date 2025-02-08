#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGroupUser {
    /// User ARN.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Path to the IAM user.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Stable and unique string identifying the IAM user.
    #[builder(into)]
    #[serde(rename = "userId")]
    pub r#user_id: Box<String>,
    /// Name of the IAM user.
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Box<String>,
}

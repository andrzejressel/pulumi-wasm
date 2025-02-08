#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClientAuthenticationSasl {
    #[builder(into, default)]
    #[serde(rename = "iam")]
    pub r#iam: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "scram")]
    pub r#scram: Box<Option<bool>>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetTableAclAccessPolicy {
    #[builder(into)]
    #[serde(rename = "expiry")]
    pub r#expiry: Box<String>,
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<String>,
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Box<String>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceAdminSettings {
    #[builder(into, default)]
    #[serde(rename = "allowedEmailDomains")]
    pub r#allowed_email_domains: Box<Option<Vec<String>>>,
}

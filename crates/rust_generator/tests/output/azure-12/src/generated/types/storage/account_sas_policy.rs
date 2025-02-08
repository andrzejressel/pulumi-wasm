#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountSasPolicy {
    /// The SAS expiration action. The only possible value is `Log` at this moment. Defaults to `Log`.
    #[builder(into, default)]
    #[serde(rename = "expirationAction")]
    pub r#expiration_action: Box<Option<String>>,
    /// The SAS expiration period in format of `DD.HH:MM:SS`.
    #[builder(into)]
    #[serde(rename = "expirationPeriod")]
    pub r#expiration_period: Box<String>,
}

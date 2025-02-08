#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConfigQuota {
    /// Quota for the Signup endpoint, if overwritten. Signup quota is measured in sign ups per project per hour per IP. None of quota, startTime, or quotaDuration can be skipped.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "signUpQuotaConfig")]
    pub r#sign_up_quota_config: Box<Option<super::super::types::identityplatform::ConfigQuotaSignUpQuotaConfig>>,
}

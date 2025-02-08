#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct UserPoolUserAttributeUpdateSettings {
    /// A list of attributes requiring verification before update. If set, the provided value(s) must also be set in `auto_verified_attributes`. Valid values: `email`, `phone_number`.
    #[builder(into)]
    #[serde(rename = "attributesRequireVerificationBeforeUpdates")]
    pub r#attributes_require_verification_before_updates: Box<Vec<String>>,
}

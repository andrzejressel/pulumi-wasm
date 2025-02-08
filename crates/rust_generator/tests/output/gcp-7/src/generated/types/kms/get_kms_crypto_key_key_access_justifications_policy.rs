#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetKmsCryptoKeyKeyAccessJustificationsPolicy {
    /// The list of allowed reasons for access to this CryptoKey. Zero allowed
    /// access reasons means all encrypt, decrypt, and sign operations for
    /// this CryptoKey will fail.
    #[builder(into)]
    #[serde(rename = "allowedAccessReasons")]
    pub r#allowed_access_reasons: Box<Vec<String>>,
}

#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceVerifiedAccessTrustProvider {
    /// A description for the AWS Verified Access Instance.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The type of device-based trust provider.
    #[builder(into, default)]
    #[serde(rename = "deviceTrustProviderType")]
    pub r#device_trust_provider_type: Box<Option<String>>,
    /// The type of trust provider (user- or device-based).
    #[builder(into, default)]
    #[serde(rename = "trustProviderType")]
    pub r#trust_provider_type: Box<Option<String>>,
    /// The type of user-based trust provider.
    #[builder(into, default)]
    #[serde(rename = "userTrustProviderType")]
    pub r#user_trust_provider_type: Box<Option<String>>,
    /// The ID of the trust provider.
    #[builder(into, default)]
    #[serde(rename = "verifiedAccessTrustProviderId")]
    pub r#verified_access_trust_provider_id: Box<Option<String>>,
}
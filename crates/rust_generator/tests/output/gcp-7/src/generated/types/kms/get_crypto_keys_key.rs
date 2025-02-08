#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetCryptoKeysKey {
    /// The resource name of the backend environment associated with all CryptoKeyVersions within this CryptoKey.
    /// The resource name is in the format "projects/*/locations/*/ekmConnections/*" and only applies to "EXTERNAL_VPC" keys.
    #[builder(into)]
    #[serde(rename = "cryptoKeyBackend")]
    pub r#crypto_key_backend: Box<String>,
    /// The period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED.
    /// If not specified at creation time, the default duration is 30 days.
    #[builder(into)]
    #[serde(rename = "destroyScheduledDuration")]
    pub r#destroy_scheduled_duration: Box<String>,
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: Box<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Whether this key may contain imported versions only.
    #[builder(into)]
    #[serde(rename = "importOnly")]
    pub r#import_only: Box<bool>,
    /// The policy used for Key Access Justifications Policy Enforcement. If this
    /// field is present and this key is enrolled in Key Access Justifications
    /// Policy Enforcement, the policy will be evaluated in encrypt, decrypt, and
    /// sign operations, and the operation will fail if rejected by the policy. The
    /// policy is defined by specifying zero or more allowed justification codes.
    /// https://cloud.google.com/assured-workloads/key-access-justifications/docs/justification-codes
    /// By default, this field is absent, and all justification codes are allowed.
    /// This field is currently in beta and is subject to change.
    #[builder(into)]
    #[serde(rename = "keyAccessJustificationsPolicies")]
    pub r#key_access_justifications_policies: Box<Vec<super::super::types::kms::GetCryptoKeysKeyKeyAccessJustificationsPolicy>>,
    /// The key ring that the keys belongs to. Format: 'projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}'.,
    #[builder(into, default)]
    #[serde(rename = "keyRing")]
    pub r#key_ring: Box<Option<String>>,
    /// Labels with user-defined metadata to apply to this resource.
    /// 
    /// 
    /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
    /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// The resource name for the CryptoKey.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// A copy of the primary CryptoKeyVersion that will be used by cryptoKeys.encrypt when this CryptoKey is given in EncryptRequest.name.
    /// Keys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be unset.
    #[builder(into)]
    #[serde(rename = "primaries")]
    pub r#primaries: Box<Vec<super::super::types::kms::GetCryptoKeysKeyPrimary>>,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: Box<std::collections::HashMap<String, String>>,
    /// The immutable purpose of this CryptoKey. See the
    /// [purpose reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys#CryptoKeyPurpose)
    /// for possible inputs.
    /// Default value is "ENCRYPT_DECRYPT".
    #[builder(into)]
    #[serde(rename = "purpose")]
    pub r#purpose: Box<String>,
    /// Every time this period passes, generate a new CryptoKeyVersion and set it as the primary.
    /// The first rotation will take place after the specified period. The rotation period has
    /// the format of a decimal number with up to 9 fractional digits, followed by the
    /// letter 's' (seconds). It must be greater than a day (ie, 86400).
    #[builder(into)]
    #[serde(rename = "rotationPeriod")]
    pub r#rotation_period: Box<String>,
    /// If set to true, the request will create a CryptoKey without any CryptoKeyVersions.
    /// You must use the 'google_kms_crypto_key_version' resource to create a new CryptoKeyVersion
    /// or 'google_kms_key_ring_import_job' resource to import the CryptoKeyVersion.
    #[builder(into)]
    #[serde(rename = "skipInitialVersionCreation")]
    pub r#skip_initial_version_creation: Box<bool>,
    /// A template describing settings for new crypto key versions.
    #[builder(into)]
    #[serde(rename = "versionTemplates")]
    pub r#version_templates: Box<Vec<super::super::types::kms::GetCryptoKeysKeyVersionTemplate>>,
}

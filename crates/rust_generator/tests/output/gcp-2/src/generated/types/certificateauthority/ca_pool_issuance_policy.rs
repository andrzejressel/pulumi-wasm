#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CaPoolIssuancePolicy {
    /// IssuanceModes specifies the allowed ways in which Certificates may be requested from this CaPool.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "allowedIssuanceModes")]
    pub r#allowed_issuance_modes: Box<Option<super::super::types::certificateauthority::CaPoolIssuancePolicyAllowedIssuanceModes>>,
    /// If any AllowedKeyType is specified, then the certificate request's public key must match one of the key types listed here.
    /// Otherwise, any key may be used.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "allowedKeyTypes")]
    pub r#allowed_key_types: Box<Option<Vec<super::super::types::certificateauthority::CaPoolIssuancePolicyAllowedKeyType>>>,
    /// A set of X.509 values that will be applied to all certificates issued through this CaPool. If a certificate request
    /// includes conflicting values for the same properties, they will be overwritten by the values defined here. If a certificate
    /// request uses a CertificateTemplate that defines conflicting predefinedValues for the same properties, the certificate
    /// issuance request will fail.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "baselineValues")]
    pub r#baseline_values: Box<Option<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValues>>,
    /// Describes constraints on identities that may appear in Certificates issued through this CaPool.
    /// If this is omitted, then this CaPool will not add restrictions on a certificate's identity.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "identityConstraints")]
    pub r#identity_constraints: Box<Option<super::super::types::certificateauthority::CaPoolIssuancePolicyIdentityConstraints>>,
    /// The maximum lifetime allowed for issued Certificates. Note that if the issuing CertificateAuthority
    /// expires before a Certificate's requested maximumLifetime, the effective lifetime will be explicitly truncated to match it.
    #[builder(into, default)]
    #[serde(rename = "maximumLifetime")]
    pub r#maximum_lifetime: Box<Option<String>>,
}

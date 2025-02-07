#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CaPoolIssuancePolicyBaselineValues {
    /// Specifies an X.509 extension, which may be used in different parts of X.509 objects like certificates, CSRs, and CRLs.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "additionalExtensions")]
    pub r#additional_extensions: Box<Option<Vec<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesAdditionalExtension>>>,
    /// Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the
    /// "Authority Information Access" extension in the certificate.
    #[builder(into, default)]
    #[serde(rename = "aiaOcspServers")]
    pub r#aia_ocsp_servers: Box<Option<Vec<String>>>,
    /// Describes values that are relevant in a CA certificate.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "caOptions")]
    pub r#ca_options: Box<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesCaOptions>,
    /// Indicates the intended use for keys that correspond to a certificate.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "keyUsage")]
    pub r#key_usage: Box<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesKeyUsage>,
    /// Describes the X.509 name constraints extension.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "nameConstraints")]
    pub r#name_constraints: Box<Option<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesNameConstraints>>,
    /// Describes the X.509 certificate policy object identifiers, per https://tools.ietf.org/html/rfc5280#section-4.2.1.4.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "policyIds")]
    pub r#policy_ids: Box<Option<Vec<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesPolicyId>>>,
}

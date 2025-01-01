#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAuthorityConfigX509Config {
    /// Specifies an X.509 extension, which may be used in different parts of X.509 objects like certificates, CSRs, and CRLs.
    #[builder(into)]
    #[serde(rename = "additionalExtensions")]
    pub r#additional_extensions: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigAdditionalExtension>>,
    /// Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the
    /// "Authority Information Access" extension in the certificate.
    #[builder(into)]
    #[serde(rename = "aiaOcspServers")]
    pub r#aia_ocsp_servers: Box<Vec<String>>,
    /// Describes values that are relevant in a CA certificate.
    #[builder(into)]
    #[serde(rename = "caOptions")]
    pub r#ca_options: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigCaOption>>,
    /// Indicates the intended use for keys that correspond to a certificate.
    #[builder(into)]
    #[serde(rename = "keyUsages")]
    pub r#key_usages: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigKeyUsage>>,
    /// Describes the X.509 name constraints extension.
    #[builder(into)]
    #[serde(rename = "nameConstraints")]
    pub r#name_constraints: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigNameConstraint>>,
    /// Describes the X.509 certificate policy object identifiers, per https://tools.ietf.org/html/rfc5280#section-4.2.1.4.
    #[builder(into)]
    #[serde(rename = "policyIds")]
    pub r#policy_ids: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigPolicyId>>,
}

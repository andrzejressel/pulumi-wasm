#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthorityConfigX509Config {
    /// Specifies an X.509 extension, which may be used in different parts of X.509 objects like certificates, CSRs, and CRLs.
    #[builder(into, default)]
    #[serde(rename = "additionalExtensions")]
    pub r#additional_extensions: Box<Option<Vec<super::super::types::certificateauthority::AuthorityConfigX509ConfigAdditionalExtension>>>,
    /// Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the
    /// "Authority Information Access" extension in the certificate.
    #[builder(into, default)]
    #[serde(rename = "aiaOcspServers")]
    pub r#aia_ocsp_servers: Box<Option<Vec<String>>>,
    /// Describes values that are relevant in a CA certificate.
    #[builder(into)]
    #[serde(rename = "caOptions")]
    pub r#ca_options: Box<super::super::types::certificateauthority::AuthorityConfigX509ConfigCaOptions>,
    /// Indicates the intended use for keys that correspond to a certificate.
    #[builder(into)]
    #[serde(rename = "keyUsage")]
    pub r#key_usage: Box<super::super::types::certificateauthority::AuthorityConfigX509ConfigKeyUsage>,
    /// Describes the X.509 name constraints extension.
    #[builder(into, default)]
    #[serde(rename = "nameConstraints")]
    pub r#name_constraints: Box<Option<super::super::types::certificateauthority::AuthorityConfigX509ConfigNameConstraints>>,
    /// Describes the X.509 certificate policy object identifiers, per https://tools.ietf.org/html/rfc5280#section-4.2.1.4.
    #[builder(into, default)]
    #[serde(rename = "policyIds")]
    pub r#policy_ids: Box<Option<Vec<super::super::types::certificateauthority::AuthorityConfigX509ConfigPolicyId>>>,
}

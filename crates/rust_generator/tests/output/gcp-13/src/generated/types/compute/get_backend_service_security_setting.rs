#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBackendServiceSecuritySetting {
    /// The configuration needed to generate a signature for access to private storage buckets that support AWS's Signature Version 4 for authentication.
    /// Allowed only for INTERNET_IP_PORT and INTERNET_FQDN_PORT NEG backends.
    #[builder(into)]
    #[serde(rename = "awsV4Authentications")]
    pub r#aws_v_4_authentications: Box<Vec<super::super::types::compute::GetBackendServiceSecuritySettingAwsV4Authentication>>,
    /// ClientTlsPolicy is a resource that specifies how a client should authenticate
    /// connections to backends of a service. This resource itself does not affect
    /// configuration unless it is attached to a backend service resource.
    #[builder(into)]
    #[serde(rename = "clientTlsPolicy")]
    pub r#client_tls_policy: Box<String>,
    /// A list of alternate names to verify the subject identity in the certificate.
    /// If specified, the client will verify that the server certificate's subject
    /// alt name matches one of the specified values.
    #[builder(into)]
    #[serde(rename = "subjectAltNames")]
    pub r#subject_alt_names: Box<Vec<String>>,
}

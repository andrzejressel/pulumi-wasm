#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackendServiceSecuritySettings {
    /// The configuration needed to generate a signature for access to private storage buckets that support AWS's Signature Version 4 for authentication.
    /// Allowed only for INTERNET_IP_PORT and INTERNET_FQDN_PORT NEG backends.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_aws_v4_authentication"></a>The `aws_v4_authentication` block supports:
    #[builder(into, default)]
    #[serde(rename = "awsV4Authentication")]
    pub r#aws_v_4_authentication: Box<Option<super::super::types::compute::BackendServiceSecuritySettingsAwsV4Authentication>>,
    /// ClientTlsPolicy is a resource that specifies how a client should authenticate
    /// connections to backends of a service. This resource itself does not affect
    /// configuration unless it is attached to a backend service resource.
    #[builder(into, default)]
    #[serde(rename = "clientTlsPolicy")]
    pub r#client_tls_policy: Box<Option<String>>,
    /// A list of alternate names to verify the subject identity in the certificate.
    /// If specified, the client will verify that the server certificate's subject
    /// alt name matches one of the specified values.
    #[builder(into, default)]
    #[serde(rename = "subjectAltNames")]
    pub r#subject_alt_names: Box<Option<Vec<String>>>,
}

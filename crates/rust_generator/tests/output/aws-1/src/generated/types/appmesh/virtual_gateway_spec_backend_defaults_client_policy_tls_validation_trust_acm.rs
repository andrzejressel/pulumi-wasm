#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrustAcm {
    /// One or more ACM ARNs.
    #[builder(into)]
    #[serde(rename = "certificateAuthorityArns")]
    pub r#certificate_authority_arns: Box<Vec<String>>,
}

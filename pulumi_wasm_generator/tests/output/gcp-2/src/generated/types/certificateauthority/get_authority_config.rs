#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAuthorityConfig {
    /// Specifies some of the values in a certificate that are related to the subject.
    #[builder(into)]
    #[serde(rename = "subjectConfigs")]
    pub r#subject_configs: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigSubjectConfig>>,
    /// When specified this provides a custom SKI to be used in the certificate. This should only be used to maintain a SKI of an existing CA originally created outside CA service, which was not generated using method (1) described in RFC 5280 section 4.2.1.2..
    #[builder(into)]
    #[serde(rename = "subjectKeyIds")]
    pub r#subject_key_ids: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigSubjectKeyId>>,
    /// Describes how some of the technical X.509 fields in a certificate should be populated.
    #[builder(into)]
    #[serde(rename = "x509Configs")]
    pub r#x_509_configs: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigX509Config>>,
}

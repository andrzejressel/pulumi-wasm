#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClientAuthenticationTls {
    /// List of ACM Certificate Authority Amazon Resource Names (ARNs).
    #[builder(into, default)]
    #[serde(rename = "certificateAuthorityArns")]
    pub r#certificate_authority_arns: Box<Option<Vec<String>>>,
}

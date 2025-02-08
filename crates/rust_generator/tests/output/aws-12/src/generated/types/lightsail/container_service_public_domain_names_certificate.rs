#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContainerServicePublicDomainNamesCertificate {
    #[builder(into)]
    #[serde(rename = "certificateName")]
    pub r#certificate_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "domainNames")]
    pub r#domain_names: Box<Vec<String>>,
}

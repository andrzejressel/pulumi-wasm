#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateDomainValidationOption {
    /// Fully qualified domain name (FQDN) in the certificate.
    #[builder(into, default)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<Option<String>>,
    /// The name of the DNS record to create to validate the certificate
    #[builder(into, default)]
    #[serde(rename = "resourceRecordName")]
    pub r#resource_record_name: Box<Option<String>>,
    /// The type of DNS record to create
    #[builder(into, default)]
    #[serde(rename = "resourceRecordType")]
    pub r#resource_record_type: Box<Option<String>>,
    /// The value the DNS record needs to have
    #[builder(into, default)]
    #[serde(rename = "resourceRecordValue")]
    pub r#resource_record_value: Box<Option<String>>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EmailServiceDomainVerificationRecord {
    /// (Optional) An `dkim2` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "dkim2s")]
    pub r#dkim_2_s: Box<Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDkim2>>>,
    /// (Optional) An `dkim` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "dkims")]
    pub r#dkims: Box<Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDkim>>>,
    /// (Optional) An `dmarc` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "dmarcs")]
    pub r#dmarcs: Box<Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDmarc>>>,
    /// (Optional) An `domain` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "domains")]
    pub r#domains: Box<Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDomain>>>,
    /// (Optional) An `spf` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "spfs")]
    pub r#spfs: Box<Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordSpf>>>,
}

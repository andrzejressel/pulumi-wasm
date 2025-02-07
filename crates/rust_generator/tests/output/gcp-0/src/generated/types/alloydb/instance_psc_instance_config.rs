#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstancePscInstanceConfig {
    /// List of consumer projects that are allowed to create PSC endpoints to service-attachments to this instance.
    /// These should be specified as project numbers only.
    #[builder(into, default)]
    #[serde(rename = "allowedConsumerProjects")]
    pub r#allowed_consumer_projects: Box<Option<Vec<String>>>,
    /// (Output)
    /// The DNS name of the instance for PSC connectivity.
    /// Name convention: <uid>.<uid>.<region>.alloydb-psc.goog
    #[builder(into, default)]
    #[serde(rename = "pscDnsName")]
    pub r#psc_dns_name: Box<Option<String>>,
    /// (Output)
    /// The service attachment created when Private Service Connect (PSC) is enabled for the instance.
    /// The name of the resource will be in the format of
    /// `projects/<alloydb-tenant-project-number>/regions/<region-name>/serviceAttachments/<service-attachment-name>`
    #[builder(into, default)]
    #[serde(rename = "serviceAttachmentLink")]
    pub r#service_attachment_link: Box<Option<String>>,
}

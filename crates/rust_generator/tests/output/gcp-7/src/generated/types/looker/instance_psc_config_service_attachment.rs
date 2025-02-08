#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstancePscConfigServiceAttachment {
    /// (Output)
    /// Status of the service attachment connection.
    #[builder(into, default)]
    #[serde(rename = "connectionStatus")]
    pub r#connection_status: Box<Option<String>>,
    /// Fully qualified domain name that will be used in the private DNS record created for the service attachment.
    #[builder(into, default)]
    #[serde(rename = "localFqdn")]
    pub r#local_fqdn: Box<Option<String>>,
    /// URI of the service attachment to connect to.
    #[builder(into, default)]
    #[serde(rename = "targetServiceAttachmentUri")]
    pub r#target_service_attachment_uri: Box<Option<String>>,
}

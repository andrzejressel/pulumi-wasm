#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstancePscConfig {
    /// List of VPCs that are allowed ingress into the Looker instance.
    #[builder(into, default)]
    #[serde(rename = "allowedVpcs")]
    pub r#allowed_vpcs: Box<Option<Vec<String>>>,
    /// (Output)
    /// URI of the Looker service attachment.
    #[builder(into, default)]
    #[serde(rename = "lookerServiceAttachmentUri")]
    pub r#looker_service_attachment_uri: Box<Option<String>>,
    /// List of egress service attachment configurations.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "serviceAttachments")]
    pub r#service_attachments: Box<Option<Vec<super::super::types::looker::InstancePscConfigServiceAttachment>>>,
}

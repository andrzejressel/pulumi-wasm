#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkInterfaceAttachment {
    #[builder(into)]
    #[serde(rename = "attachmentId")]
    pub r#attachment_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "deviceIndex")]
    pub r#device_index: Box<i32>,
    #[builder(into)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "instanceOwnerId")]
    pub r#instance_owner_id: Box<String>,
}

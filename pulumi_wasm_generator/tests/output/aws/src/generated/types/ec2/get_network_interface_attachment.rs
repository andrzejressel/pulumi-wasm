#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
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

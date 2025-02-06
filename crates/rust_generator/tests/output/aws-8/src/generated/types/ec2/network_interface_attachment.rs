#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkInterfaceAttachment {
    #[builder(into, default)]
    #[serde(rename = "attachmentId")]
    pub r#attachment_id: Box<Option<String>>,
    /// Integer to define the devices index.
    #[builder(into)]
    #[serde(rename = "deviceIndex")]
    pub r#device_index: Box<i32>,
    /// ID of the instance to attach to.
    #[builder(into)]
    #[serde(rename = "instance")]
    pub r#instance: Box<String>,
}

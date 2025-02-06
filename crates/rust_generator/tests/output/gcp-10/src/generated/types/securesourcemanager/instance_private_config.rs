#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstancePrivateConfig {
    /// CA pool resource, resource must in the format of `projects/{project}/locations/{location}/caPools/{ca_pool}`.
    #[builder(into)]
    #[serde(rename = "caPool")]
    pub r#ca_pool: Box<String>,
    /// (Output)
    /// Service Attachment for HTTP, resource is in the format of `projects/{project}/regions/{region}/serviceAttachments/{service_attachment}`.
    #[builder(into, default)]
    #[serde(rename = "httpServiceAttachment")]
    pub r#http_service_attachment: Box<Option<String>>,
    /// 'Indicate if it's private instance.'
    #[builder(into)]
    #[serde(rename = "isPrivate")]
    pub r#is_private: Box<bool>,
    /// (Output)
    /// Service Attachment for SSH, resource is in the format of `projects/{project}/regions/{region}/serviceAttachments/{service_attachment}`.
    #[builder(into, default)]
    #[serde(rename = "sshServiceAttachment")]
    pub r#ssh_service_attachment: Box<Option<String>>,
}

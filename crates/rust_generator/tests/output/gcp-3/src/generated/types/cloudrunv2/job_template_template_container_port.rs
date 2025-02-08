#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateTemplateContainerPort {
    /// Port number the container listens on. This must be a valid TCP port number, 0 < containerPort < 65536.
    #[builder(into, default)]
    #[serde(rename = "containerPort")]
    pub r#container_port: Box<Option<i32>>,
    /// If specified, used to specify which protocol to use. Allowed values are "http1" and "h2c".
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}

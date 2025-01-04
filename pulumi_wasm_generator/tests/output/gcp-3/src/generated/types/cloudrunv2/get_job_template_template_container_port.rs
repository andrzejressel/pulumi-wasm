#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobTemplateTemplateContainerPort {
    /// Port number the container listens on. This must be a valid TCP port number, 0 < containerPort < 65536.
    #[builder(into)]
    #[serde(rename = "containerPort")]
    pub r#container_port: Box<i32>,
    /// The name of the Cloud Run v2 Job.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

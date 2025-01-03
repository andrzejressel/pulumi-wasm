#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResponsePolicyNetwork {
    /// The fully qualified URL of the VPC network to bind to.
    /// This should be formatted like
    /// `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`
    #[builder(into)]
    #[serde(rename = "networkUrl")]
    pub r#network_url: Box<String>,
}

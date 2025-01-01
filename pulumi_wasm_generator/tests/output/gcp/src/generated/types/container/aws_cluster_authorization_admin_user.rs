#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsClusterAuthorizationAdminUser {
    /// The name of the user, e.g. `my-gcp-id@gmail.com`.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}

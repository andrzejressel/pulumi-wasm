#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterAuthorizationAdminUsers {
    /// An active Google username.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}

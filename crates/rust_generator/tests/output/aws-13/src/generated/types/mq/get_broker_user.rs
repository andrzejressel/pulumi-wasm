#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBrokerUser {
    #[builder(into)]
    #[serde(rename = "consoleAccess")]
    pub r#console_access: Box<bool>,
    #[builder(into)]
    #[serde(rename = "groups")]
    pub r#groups: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "replicationUser")]
    pub r#replication_user: Box<bool>,
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}

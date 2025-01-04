#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SparkClusterGateway {
    /// The password used for the Ambari Portal.
    /// 
    /// > **NOTE:** This password must be different from the one used for the `head_node`, `worker_node` and `zookeeper_node` roles.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The username used for the Ambari Portal. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}

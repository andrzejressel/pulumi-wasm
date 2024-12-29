#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterIdentity {
    /// Nested block containing [OpenID Connect](https://openid.net/connect/) identity provider information for the cluster. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "oidcs")]
    pub r#oidcs: Box<Option<Vec<super::super::types::eks::ClusterIdentityOidc>>>,
}

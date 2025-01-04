#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalClusterSecurityConfig {
    /// Configures user access to the Bare Metal User cluster.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "authorization")]
    pub r#authorization: Box<Option<super::super::types::gkeonprem::BareMetalClusterSecurityConfigAuthorization>>,
}

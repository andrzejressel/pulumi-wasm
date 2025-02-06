#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustGatewaySettingsFips {
    /// Only allow FIPS-compliant TLS configuration.
    #[builder(into, default)]
    #[serde(rename = "tls")]
    pub r#tls: Box<Option<bool>>,
}

#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewaySslProfileSslPolicy {
    #[builder(into, default)]
    #[serde(rename = "cipherSuites")]
    pub r#cipher_suites: Box<Option<Vec<String>>>,
    /// A list of SSL Protocols which should be disabled on this Application Gateway. Possible values are `TLSv1_0`, `TLSv1_1`, `TLSv1_2` and `TLSv1_3`.
    /// 
    /// > **NOTE:** `disabled_protocols` cannot be set when `policy_name` or `policy_type` are set.
    #[builder(into, default)]
    #[serde(rename = "disabledProtocols")]
    pub r#disabled_protocols: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "minProtocolVersion")]
    pub r#min_protocol_version: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "policyName")]
    pub r#policy_name: Box<Option<String>>,
    /// The Type of the Policy. Possible values are `Predefined`, `Custom` and `CustomV2`.
    /// 
    /// > **NOTE:** `policy_type` is Required when `policy_name` is set - cannot be set if `disabled_protocols` is set.
    #[builder(into, default)]
    #[serde(rename = "policyType")]
    pub r#policy_type: Box<Option<String>>,
}

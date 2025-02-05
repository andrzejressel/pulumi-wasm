#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionSecurityPolicyRuleMatch {
    /// The configuration options available when specifying versionedExpr.
    /// This field must be specified if versionedExpr is specified and cannot be specified if versionedExpr is not specified.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "config")]
    pub r#config: Box<Option<super::super::types::compute::RegionSecurityPolicyRuleMatchConfig>>,
    /// User defined CEVAL expression. A CEVAL expression is used to specify match criteria such as origin.ip, source.region_code and contents in the request header.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "expr")]
    pub r#expr: Box<Option<super::super::types::compute::RegionSecurityPolicyRuleMatchExpr>>,
    /// Preconfigured versioned expression. If this field is specified, config must also be specified.
    /// Available preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding srcIpRange field in config.
    /// Possible values are: `SRC_IPS_V1`.
    #[builder(into, default)]
    #[serde(rename = "versionedExpr")]
    pub r#versioned_expr: Box<Option<String>>,
}

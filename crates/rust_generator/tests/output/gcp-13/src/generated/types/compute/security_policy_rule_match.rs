#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SecurityPolicyRuleMatch {
    /// The configuration options available when specifying versionedExpr.
    /// This field must be specified if versionedExpr is specified and cannot be specified if versionedExpr is not specified.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "config")]
    pub r#config: Box<Option<super::super::types::compute::SecurityPolicyRuleMatchConfig>>,
    /// User defined CEVAL expression. A CEVAL expression is used to specify match criteria such as origin.ip, source.region_code and contents in the request header.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "expr")]
    pub r#expr: Box<Option<super::super::types::compute::SecurityPolicyRuleMatchExpr>>,
    /// The configuration options available when specifying a user defined CEVAL expression (i.e., 'expr').
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "exprOptions")]
    pub r#expr_options: Box<Option<super::super::types::compute::SecurityPolicyRuleMatchExprOptions>>,
    /// Preconfigured versioned expression. If this field is specified, config must also be specified.
    /// Available preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding srcIpRange field in config.
    /// Possible values are: `SRC_IPS_V1`.
    #[builder(into, default)]
    #[serde(rename = "versionedExpr")]
    pub r#versioned_expr: Box<Option<String>>,
}

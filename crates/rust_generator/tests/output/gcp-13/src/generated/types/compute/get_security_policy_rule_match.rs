#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetSecurityPolicyRuleMatch {
    /// The configuration options available when specifying versioned_expr. This field must be specified if versioned_expr is specified and cannot be specified if versioned_expr is not specified.
    #[builder(into)]
    #[serde(rename = "configs")]
    pub r#configs: Box<Vec<super::super::types::compute::GetSecurityPolicyRuleMatchConfig>>,
    /// The configuration options available when specifying a user defined CEVAL expression (i.e., 'expr').
    #[builder(into)]
    #[serde(rename = "exprOptions")]
    pub r#expr_options: Box<Vec<super::super::types::compute::GetSecurityPolicyRuleMatchExprOption>>,
    /// User defined CEVAL expression. A CEVAL expression is used to specify match criteria such as origin.ip, source.region_code and contents in the request header.
    #[builder(into)]
    #[serde(rename = "exprs")]
    pub r#exprs: Box<Vec<super::super::types::compute::GetSecurityPolicyRuleMatchExpr>>,
    /// Predefined rule expression. If this field is specified, config must also be specified. Available options:   SRC_IPS_V1: Must specify the corresponding src_ip_ranges field in config.
    #[builder(into)]
    #[serde(rename = "versionedExpr")]
    pub r#versioned_expr: Box<String>,
}

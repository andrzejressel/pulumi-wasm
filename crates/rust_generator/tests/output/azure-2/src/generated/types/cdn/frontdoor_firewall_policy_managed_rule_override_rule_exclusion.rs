#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorFirewallPolicyManagedRuleOverrideRuleExclusion {
    /// The variable type to be excluded. Possible values are `QueryStringArgNames`, `RequestBodyPostArgNames`, `RequestCookieNames`, `RequestHeaderNames`, `RequestBodyJsonArgNames`
    /// 
    /// > **NOTE:** `RequestBodyJsonArgNames` is only available on Default Rule Set (DRS) 2.0 or later
    #[builder(into)]
    #[serde(rename = "matchVariable")]
    pub r#match_variable: Box<String>,
    /// Comparison operator to apply to the selector when specifying which elements in the collection this exclusion applies to. Possible values are: `Equals`, `Contains`, `StartsWith`, `EndsWith`, `EqualsAny`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Selector for the value in the `match_variable` attribute this exclusion applies to.
    /// 
    /// > **NOTE:** `selector` must be set to `*` if `operator` is set to `EqualsAny`.
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: Box<String>,
}

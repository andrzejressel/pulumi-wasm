#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicySpecRule {
    /// Setting this to `"TRUE"` means that all values are allowed. This field can be set only in Policies for list constraints.
    #[builder(into, default)]
    #[serde(rename = "allowAll")]
    pub r#allow_all: Box<Option<String>>,
    /// A condition which determines whether this rule is used in the evaluation of the policy. When set, the `expression` field in the `Expr' must include from 1 to 10 subexpressions, joined by the "||" or "&&" operators. Each subexpression must be of the form "resource.matchTag('/tag_key_short_name, 'tag_value_short_name')". or "resource.matchTagId('tagKeys/key_id', 'tagValues/value_id')". where key_name and value_name are the resource names for Label Keys and Values. These names are available from the Tag Manager Service. An example expression is: "resource.matchTag('123456789/environment, 'prod')". or "resource.matchTagId('tagKeys/123', 'tagValues/456')".
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<super::super::types::orgpolicy::PolicySpecRuleCondition>>,
    /// Setting this to `"TRUE"` means that all values are denied. This field can be set only in Policies for list constraints.
    #[builder(into, default)]
    #[serde(rename = "denyAll")]
    pub r#deny_all: Box<Option<String>>,
    /// If `"TRUE"`, then the `Policy` is enforced. If `"FALSE"`, then any configuration is acceptable. This field can be set only in Policies for boolean constraints.
    #[builder(into, default)]
    #[serde(rename = "enforce")]
    pub r#enforce: Box<Option<String>>,
    /// Optional. Required for Managed Constraints if parameters defined in constraints. Pass parameter values when policy enforcement is enabled. Ensure that parameter value types match those defined in the constraint definition. For example: { \"allowedLocations\" : [\"us-east1\", \"us-west1\"], \"allowAll\" : true }
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<String>>,
    /// List of values to be used for this policy rule. This field can be set only in policies for list constraints.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "values")]
    pub r#values: Box<Option<super::super::types::orgpolicy::PolicySpecRuleValues>>,
}

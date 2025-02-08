#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorRuleConditionsSocketAddressCondition {
    /// Specify one or more IP address ranges. If multiple IP address ranges are specified, they're evaluated using `OR` logic.
    /// 
    /// ->**NOTE:** See the `Specifying IP Address Ranges` section below on how to correctly define the `match_values` field.
    #[builder(into, default)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Box<Option<Vec<String>>>,
    /// If `true` operator becomes the opposite of its value. Possible values `true` or `false`. Defaults to `false`. Details can be found in the `Condition Operator List` below.
    #[builder(into, default)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Box<Option<bool>>,
    /// The type of match. The Possible values are `IpMatch` or `Any`. Defaults to `IPMatch`.
    /// 
    /// ->**NOTE:** If the value of the `operator` field is set to `IpMatch` then the `match_values` field is also required.
    #[builder(into, default)]
    #[serde(rename = "operator")]
    pub r#operator: Box<Option<String>>,
}

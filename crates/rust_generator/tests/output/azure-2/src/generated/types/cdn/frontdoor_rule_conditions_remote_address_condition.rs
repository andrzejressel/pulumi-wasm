#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FrontdoorRuleConditionsRemoteAddressCondition {
    /// For the IP Match or IP Not Match operators: specify one or more IP address ranges. If multiple IP address ranges are specified, they're evaluated using `OR` logic. For the Geo Match or Geo Not Match operators: specify one or more locations using their country code.
    /// 
    /// ->**NOTE:** See the `Specifying IP Address Ranges` section below on how to correctly define the `match_values` field.
    #[builder(into, default)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Box<Option<Vec<String>>>,
    /// If `true` operator becomes the opposite of its value. Possible values `true` or `false`. Defaults to `false`. Details can be found in the `Condition Operator List` below.
    #[builder(into, default)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Box<Option<bool>>,
    /// The type of the remote address to match. Possible values include `Any`, `GeoMatch` or `IPMatch`. Use the `negate_condition` to specify Not `GeoMatch` or Not `IPMatch`. Defaults to `IPMatch`.
    #[builder(into, default)]
    #[serde(rename = "operator")]
    pub r#operator: Box<Option<String>>,
}

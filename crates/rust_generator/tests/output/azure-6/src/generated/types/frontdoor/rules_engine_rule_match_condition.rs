#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesEngineRuleMatchCondition {
    /// can be set to `true` or `false` to negate the given condition. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Box<Option<bool>>,
    /// can be set to `Any`, `IPMatch`, `GeoMatch`, `Equal`, `Contains`, `LessThan`, `GreaterThan`, `LessThanOrEqual`, `GreaterThanOrEqual`, `BeginsWith` or `EndsWith`
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// match against a specific key when `variable` is set to `PostArgs` or `RequestHeader`. It cannot be used with `QueryString` and `RequestMethod`.
    #[builder(into, default)]
    #[serde(rename = "selector")]
    pub r#selector: Box<Option<String>>,
    /// can be set to one or more values out of `Lowercase`, `RemoveNulls`, `Trim`, `Uppercase`, `UrlDecode` and `UrlEncode`
    #[builder(into, default)]
    #[serde(rename = "transforms")]
    pub r#transforms: Box<Option<Vec<String>>>,
    /// (array) can contain one or more strings.
    #[builder(into, default)]
    #[serde(rename = "values")]
    pub r#values: Box<Option<Vec<String>>>,
    /// can be set to `IsMobile`, `RemoteAddr`, `RequestMethod`, `QueryString`, `PostArgs`, `RequestURI`, `RequestPath`, `RequestFilename`, `RequestFilenameExtension`,`RequestHeader`,`RequestBody` or `RequestScheme`.
    #[builder(into, default)]
    #[serde(rename = "variable")]
    pub r#variable: Box<Option<String>>,
}

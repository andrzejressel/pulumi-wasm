#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesetRuleActionParametersHeader {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[builder(into, default)]
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Name of the HTTP request header to target.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Action to perform on the HTTP request header. Available values: `remove`, `set`, `add`.
    #[builder(into, default)]
    #[serde(rename = "operation")]
    pub r#operation: Box<Option<String>>,
    /// Static value to provide as the HTTP request header value.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}

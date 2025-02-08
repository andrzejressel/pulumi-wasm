#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct V2ModelsIntentKendraConfiguration {
    /// ARN of the Amazon Kendra index that you want the AMAZON.KendraSearchIntent intent to search. The index must be in the same account and Region as the Amazon Lex bot.
    #[builder(into)]
    #[serde(rename = "kendraIndex")]
    pub r#kendra_index: Box<String>,
    /// Query filter that Amazon Lex sends to Amazon Kendra to filter the response from a query. The filter is in the format defined by Amazon Kendra. For more information, see [Filtering queries](https://docs.aws.amazon.com/kendra/latest/dg/filtering.html).
    #[builder(into, default)]
    #[serde(rename = "queryFilterString")]
    pub r#query_filter_string: Box<Option<String>>,
    /// Whether the AMAZON.KendraSearchIntent intent uses a custom query string to query the Amazon Kendra index.
    #[builder(into, default)]
    #[serde(rename = "queryFilterStringEnabled")]
    pub r#query_filter_string_enabled: Box<Option<bool>>,
}

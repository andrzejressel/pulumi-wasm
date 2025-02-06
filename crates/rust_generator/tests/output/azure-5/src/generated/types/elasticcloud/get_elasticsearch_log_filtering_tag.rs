#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetElasticsearchLogFilteringTag {
    /// The type of action which is taken when the Tag matches the `name` and `value`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// The name of the Elasticsearch resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of the Tag which should be filtered.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTagValuesValue {
    /// Creation time.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    /// User-assigned description of the TagValue.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// an identifier for the resource with format `tagValues/{{name}}`
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Namespaced name of the TagValue.
    #[builder(into)]
    #[serde(rename = "namespacedName")]
    pub r#namespaced_name: Box<String>,
    /// The resource name of the parent tagKey in format `tagKey/{name}`.
    #[builder(into)]
    #[serde(rename = "parent")]
    pub r#parent: Box<String>,
    /// User-assigned short name for TagValue. The short name should be unique for TagValues within the same parent TagKey.
    #[builder(into)]
    #[serde(rename = "shortName")]
    pub r#short_name: Box<String>,
    /// Update time.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<String>,
}

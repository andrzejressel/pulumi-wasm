#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConfigurationFeatureTimewindowFilter {
    /// The latest timestamp the feature is enabled. The timestamp must be in RFC3339 format.
    #[builder(into, default)]
    #[serde(rename = "end")]
    pub r#end: Box<Option<String>>,
    /// The earliest timestamp the feature is enabled. The timestamp must be in RFC3339 format.
    #[builder(into, default)]
    #[serde(rename = "start")]
    pub r#start: Box<Option<String>>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MetadataCategory {
    /// Specifies a list of domains for the solution content item.
    #[builder(into, default)]
    #[serde(rename = "domains")]
    pub r#domains: Box<Option<Vec<String>>>,
    /// Specifies a list of industry verticals for the solution content item.
    #[builder(into, default)]
    #[serde(rename = "verticals")]
    pub r#verticals: Box<Option<Vec<String>>>,
}

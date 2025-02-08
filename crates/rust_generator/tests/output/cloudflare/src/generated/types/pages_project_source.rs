#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PagesProjectSource {
    /// Configuration for the source of the Cloudflare Pages project.
    #[builder(into, default)]
    #[serde(rename = "config")]
    pub r#config: Box<Option<super::types::PagesProjectSourceConfig>>,
    /// Project host type.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}

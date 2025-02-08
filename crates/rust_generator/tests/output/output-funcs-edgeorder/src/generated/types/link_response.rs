#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LinkResponse {
    /// Type of link
    #[builder(into)]
    #[serde(rename = "linkType")]
    pub r#link_type: Box<String>,
    /// Url of the link
    #[builder(into)]
    #[serde(rename = "linkUrl")]
    pub r#link_url: Box<String>,
}

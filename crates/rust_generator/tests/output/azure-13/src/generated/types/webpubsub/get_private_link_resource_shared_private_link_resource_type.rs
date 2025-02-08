#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetPrivateLinkResourceSharedPrivateLinkResourceType {
    /// The description of the resource type that has been onboarded to private link service.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The  name for the resource that has been onboarded to private link service.
    #[builder(into)]
    #[serde(rename = "subresourceName")]
    pub r#subresource_name: Box<String>,
}

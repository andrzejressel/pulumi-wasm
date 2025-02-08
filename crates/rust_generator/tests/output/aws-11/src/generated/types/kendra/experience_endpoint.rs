#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExperienceEndpoint {
    /// The endpoint of your Amazon Kendra experience.
    #[builder(into, default)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Box<Option<String>>,
    /// The type of endpoint for your Amazon Kendra experience.
    #[builder(into, default)]
    #[serde(rename = "endpointType")]
    pub r#endpoint_type: Box<Option<String>>,
}

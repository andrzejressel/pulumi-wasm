#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDiscoveredServiceServiceProperty {
    /// The service project identifier that the underlying cloud resource resides in.
    #[builder(into)]
    #[serde(rename = "gcpProject")]
    pub r#gcp_project: Box<String>,
    /// The location of the discovered service.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The location that the underlying resource resides in if it is zonal.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Box<String>,
}

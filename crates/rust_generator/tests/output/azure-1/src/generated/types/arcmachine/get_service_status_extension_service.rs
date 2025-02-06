#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceStatusExtensionService {
    /// The behavior of the service when the Arc-enabled machine starts up.
    #[builder(into)]
    #[serde(rename = "startupType")]
    pub r#startup_type: Box<String>,
    /// The current status of the service.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}

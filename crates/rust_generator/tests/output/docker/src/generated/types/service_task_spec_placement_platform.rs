#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTaskSpecPlacementPlatform {
    /// The architecture, e.g. `amd64`
    #[builder(into)]
    #[serde(rename = "architecture")]
    pub r#architecture: Box<String>,
    /// The operation system, e.g. `linux`
    #[builder(into)]
    #[serde(rename = "os")]
    pub r#os: Box<String>,
}

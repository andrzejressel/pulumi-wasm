#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPrivateCloudManagementClusterStretchedClusterConfig {
    /// Zone that will remain operational when connection between the two zones is lost.
    #[builder(into)]
    #[serde(rename = "preferredLocation")]
    pub r#preferred_location: Box<String>,
    /// Additional zone for a higher level of availability and load balancing.
    #[builder(into)]
    #[serde(rename = "secondaryLocation")]
    pub r#secondary_location: Box<String>,
}

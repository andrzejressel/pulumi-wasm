#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkloadWorkloadProperty {
    /// (Output)
    /// Output only. The service project identifier that the underlying cloud resource resides in. Empty for non cloud resources.
    #[builder(into, default)]
    #[serde(rename = "gcpProject")]
    pub r#gcp_project: Box<Option<String>>,
    /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// (Output)
    /// Output only. The location that the underlying compute resource resides in if it is zonal (e.g us-west1-a).
    #[builder(into, default)]
    #[serde(rename = "zone")]
    pub r#zone: Box<Option<String>>,
}

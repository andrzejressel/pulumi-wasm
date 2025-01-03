#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetProvisioningArtifactsProvisioningArtifactDetail {
    /// Indicates whether the product version is active.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: Box<bool>,
    /// The UTC time stamp of the creation time.
    #[builder(into)]
    #[serde(rename = "createdTime")]
    pub r#created_time: Box<String>,
    /// The description of the provisioning artifact.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Information set by the administrator to provide guidance to end users about which provisioning artifacts to use.
    #[builder(into)]
    #[serde(rename = "guidance")]
    pub r#guidance: Box<String>,
    /// The identifier of the provisioning artifact.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of the provisioning artifact.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of provisioning artifact.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}

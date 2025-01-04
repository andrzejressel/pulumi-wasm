#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HciDeploymentSettingScaleUnitStorage {
    /// The configuration mode of storage. If set to `Express` and your storage is configured as per best practices based on the number of nodes in the cluster. Possible values are `Express`, `InfraOnly` and `KeepStorage`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "configurationMode")]
    pub r#configuration_mode: Box<String>,
}

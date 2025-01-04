#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HciDeploymentSettingScaleUnitOptionalService {
    /// Specifies the name of custom location. A custom location will be created after the deployment is completed. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "customLocation")]
    pub r#custom_location: Box<String>,
}

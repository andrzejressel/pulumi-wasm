#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionEksPropertyPodPropertyContainerResource {
    /// The type and quantity of the resources to reserve for the container.
    #[builder(into)]
    #[serde(rename = "limits")]
    pub r#limits: Box<std::collections::HashMap<String, String>>,
    /// The type and quantity of the resources to request for the container.
    #[builder(into)]
    #[serde(rename = "requests")]
    pub r#requests: Box<std::collections::HashMap<String, String>>,
}
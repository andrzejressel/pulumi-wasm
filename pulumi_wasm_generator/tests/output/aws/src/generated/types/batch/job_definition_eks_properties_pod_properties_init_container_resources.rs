#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobDefinitionEksPropertiesPodPropertiesInitContainerResources {
    #[builder(into, default)]
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<std::collections::HashMap<String, String>>>,
    #[builder(into, default)]
    #[serde(rename = "requests")]
    pub r#requests: Box<Option<std::collections::HashMap<String, String>>>,
}
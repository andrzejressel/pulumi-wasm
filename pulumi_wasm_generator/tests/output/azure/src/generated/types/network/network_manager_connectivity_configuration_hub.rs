#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkManagerConnectivityConfigurationHub {
    /// Specifies the resource ID used as hub in Hub And Spoke topology.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Box<String>,
    /// Specifies the resource Type used as hub in Hub And Spoke topology.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<String>,
}
#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobDefinitionEksProperties {
    /// Properties for the Kubernetes pod resources of a job. See `pod_properties` below.
    #[builder(into)]
    #[serde(rename = "podProperties")]
    pub r#pod_properties: Box<super::super::types::batch::JobDefinitionEksPropertiesPodProperties>,
}

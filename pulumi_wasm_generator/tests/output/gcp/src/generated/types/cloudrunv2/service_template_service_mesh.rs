#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTemplateServiceMesh {
    /// The Mesh resource name. For more information see https://cloud.google.com/service-mesh/docs/reference/network-services/rest/v1/projects.locations.meshes#resource:-mesh.
    #[builder(into, default)]
    #[serde(rename = "mesh")]
    pub r#mesh: Box<Option<String>>,
}

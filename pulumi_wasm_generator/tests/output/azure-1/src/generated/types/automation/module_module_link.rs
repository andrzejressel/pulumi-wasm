#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ModuleModuleLink {
    /// A `hash` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "hash")]
    pub r#hash: Box<Option<super::super::types::automation::ModuleModuleLinkHash>>,
    /// The URI of the module content (zip or nupkg).
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}

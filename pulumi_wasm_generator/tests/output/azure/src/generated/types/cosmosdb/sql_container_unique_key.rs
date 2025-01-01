#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SqlContainerUniqueKey {
    /// A list of paths to use for this unique key. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Vec<String>>,
}

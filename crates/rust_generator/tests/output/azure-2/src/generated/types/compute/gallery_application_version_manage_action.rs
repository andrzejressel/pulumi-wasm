#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GalleryApplicationVersionManageAction {
    /// The command to install the Gallery Application. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "install")]
    pub r#install: Box<String>,
    /// The command to remove the Gallery Application. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "remove")]
    pub r#remove: Box<String>,
    /// The command to update the Gallery Application. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "update")]
    pub r#update: Box<Option<String>>,
}

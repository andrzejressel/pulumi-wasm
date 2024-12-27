#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetListsList {
    /// List description.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// List identifier.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// List kind.
    #[builder(into, default)]
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
    /// The list name to target for the resource.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Number of items in list.
    #[builder(into, default)]
    #[serde(rename = "numitems")]
    pub r#numitems: Box<Option<i32>>,
}

#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThingTypeProperties {
    /// The description of the thing type.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A list of searchable thing attribute names.
    #[builder(into, default)]
    #[serde(rename = "searchableAttributes")]
    pub r#searchable_attributes: Box<Option<Vec<String>>>,
}

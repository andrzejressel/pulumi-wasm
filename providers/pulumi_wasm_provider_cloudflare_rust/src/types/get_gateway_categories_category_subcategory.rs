#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetGatewayCategoriesCategorySubcategory {
    /// True if the subcategory is in beta and subject to change.
    #[builder(into)]
    #[serde(rename = "beta")]
    pub r#beta: Box<bool>,
    /// Which account types are allowed to create policies based on this subcategory.
    #[builder(into)]
    #[serde(rename = "class")]
    pub r#class: Box<String>,
    /// A short summary of domains in the subcategory.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The identifier for this subcategory. There is only one subcategory per ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<i32>,
    /// The name of the subcategory.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
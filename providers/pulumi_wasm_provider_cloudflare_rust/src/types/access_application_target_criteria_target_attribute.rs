#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessApplicationTargetCriteriaTargetAttribute {
    /// The key of the attribute.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The values of the attribute.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

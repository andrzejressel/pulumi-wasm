#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsListItemsWithDescription {
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

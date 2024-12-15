#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct UnionCase2 {
    #[builder(into)]
    #[serde(rename = "field2")]
    pub r#field_2: Box<String>,
}

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct DirectCycle {
    #[builder(into)]
    #[serde(rename = "foo")]
    pub r#foo: Box<crate::types::DirectCycle>,
}

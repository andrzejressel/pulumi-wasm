#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct IndirectCycleS {
    #[builder(into)]
    #[serde(rename = "foo2")]
    pub r#foo_2: Box<super::types::IndirectCycleT>,
}

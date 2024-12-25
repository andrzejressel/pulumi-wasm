#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct IndirectCycleT {
    #[builder(into)]
    #[serde(rename = "foo3")]
    pub r#foo_3: Box<super::types::IndirectCycleS>,
}

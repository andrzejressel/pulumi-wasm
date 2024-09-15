#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct IndirectCycleT {
    #[builder(into)]
    #[serde(rename = "foo3")]
    pub r#foo_3: Box<crate::types::IndirectCycleS>,
}

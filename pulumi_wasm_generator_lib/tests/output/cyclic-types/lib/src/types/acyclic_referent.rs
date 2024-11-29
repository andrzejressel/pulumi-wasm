#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AcyclicReferent {
    #[builder(into)]
    #[serde(rename = "bar")]
    pub r#bar: Box<crate::types::IndirectCycleS>,
    #[builder(into)]
    #[serde(rename = "baz")]
    pub r#baz: Box<crate::types::IndirectCycleT>,
    #[builder(into)]
    #[serde(rename = "foo4")]
    pub r#foo_4: Box<crate::types::DirectCycle>,
}

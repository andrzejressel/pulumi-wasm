#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AcyclicReferent {
    #[serde(rename = "bar")]
    pub r#bar: Box<crate::types::IndirectCycleS>,
    #[serde(rename = "baz")]
    pub r#baz: Box<crate::types::IndirectCycleT>,
    #[serde(rename = "foo4")]
    pub r#foo_4: Box<crate::types::DirectCycle>,
}

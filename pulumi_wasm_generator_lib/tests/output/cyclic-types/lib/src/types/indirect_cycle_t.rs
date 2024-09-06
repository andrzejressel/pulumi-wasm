#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct IndirectCycleT {
    #[serde(rename = "foo3")]
    pub r#foo_3: Box<crate::types::IndirectCycleS>,
}

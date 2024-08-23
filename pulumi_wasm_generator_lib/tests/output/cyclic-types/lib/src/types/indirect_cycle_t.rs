#[derive(serde::Serialize)]
pub struct IndirectCycleT {
    #[serde(rename = "foo3")]
    pub r#foo_3: Box<crate::types::IndirectCycleS>,
}

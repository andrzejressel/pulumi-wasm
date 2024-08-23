#[derive(serde::Serialize)]
pub struct IndirectCycleS {
    #[serde(rename = "foo2")]
    pub r#foo_2: Box<crate::types::IndirectCycleT>,
}

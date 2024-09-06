#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct IndirectCycleS {
    #[serde(rename = "foo2")]
    pub r#foo_2: Box<crate::types::IndirectCycleT>,
}

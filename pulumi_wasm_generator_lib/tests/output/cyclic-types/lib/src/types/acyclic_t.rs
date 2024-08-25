#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AcyclicT {
    #[serde(rename = "foo6")]
    pub r#foo_6: Box<crate::types::AcyclicS>,
}

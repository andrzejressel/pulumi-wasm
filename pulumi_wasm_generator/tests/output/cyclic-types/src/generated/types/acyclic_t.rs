#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AcyclicT {
    #[builder(into)]
    #[serde(rename = "foo6")]
    pub r#foo_6: Box<super::types::AcyclicS>,
}

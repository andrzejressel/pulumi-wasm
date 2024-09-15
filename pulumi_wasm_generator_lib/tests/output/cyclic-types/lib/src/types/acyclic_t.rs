#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AcyclicT {
    #[builder(into)]
    #[serde(rename = "foo6")]
    pub r#foo_6: Box<crate::types::AcyclicS>,
}

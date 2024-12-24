#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AcyclicS {
    #[builder(into)]
    #[serde(rename = "foo5")]
    pub r#foo_5: Box<String>,
}

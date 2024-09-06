#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AcyclicS {
    #[serde(rename = "foo5")]
    pub r#foo_5: Box<String>,
}

#[derive(serde::Serialize)]
pub struct SpectrumApplicationOriginPortRange {
    #[serde(rename = "end")]
    pub r#end: Box<i32>,
    #[serde(rename = "start")]
    pub r#start: Box<i32>,
}

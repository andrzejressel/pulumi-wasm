#[derive(serde::Serialize)]
pub struct SpectrumApplicationOriginPortRange {
    /// Upper bound of the origin port range.
    #[serde(rename = "end")]
    pub r#end: Box<i32>,
    /// Lower bound of the origin port range.
    #[serde(rename = "start")]
    pub r#start: Box<i32>,
}

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SpectrumApplicationOriginPortRange {
    /// Upper bound of the origin port range.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: Box<i32>,
    /// Lower bound of the origin port range.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Box<i32>,
}

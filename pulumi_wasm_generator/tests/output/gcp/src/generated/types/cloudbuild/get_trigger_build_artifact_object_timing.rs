#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerBuildArtifactObjectTiming {
    /// End of time span.
    /// 
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to
    /// nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<String>,
    /// Start of time span.
    /// 
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to
    /// nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}

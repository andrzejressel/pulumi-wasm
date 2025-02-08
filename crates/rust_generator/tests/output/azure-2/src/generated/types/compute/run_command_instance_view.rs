#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RunCommandInstanceView {
    #[builder(into, default)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "errorMessage")]
    pub r#error_message: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "executionMessage")]
    pub r#execution_message: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "executionState")]
    pub r#execution_state: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "exitCode")]
    pub r#exit_code: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "output")]
    pub r#output: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct QueueHttpTargetHeaderOverrideHeader {
    /// The Key of the header.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The Value of the header.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

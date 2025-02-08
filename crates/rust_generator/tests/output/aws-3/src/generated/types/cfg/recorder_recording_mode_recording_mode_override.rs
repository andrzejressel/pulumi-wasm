#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecorderRecordingModeRecordingModeOverride {
    /// A description you provide of the override.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The recording frequency for the resources in the override block. `CONTINUOUS` or `DAILY`.
    #[builder(into)]
    #[serde(rename = "recordingFrequency")]
    pub r#recording_frequency: Box<String>,
    /// A list that specifies the types of AWS resources for which the override applies to.  See [restrictions in the AWS Docs](https://docs.aws.amazon.com/config/latest/APIReference/API_RecordingModeOverride.html)
    #[builder(into)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Box<Vec<String>>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamProcessorInputKinesisVideoStream {
    /// ARN of the Kinesis video stream stream that streams the source video.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
}

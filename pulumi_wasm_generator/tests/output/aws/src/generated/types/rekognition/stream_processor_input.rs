#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamProcessorInput {
    /// Kinesis input stream. See `kinesis_video_stream`.
    #[builder(into, default)]
    #[serde(rename = "kinesisVideoStream")]
    pub r#kinesis_video_stream: Box<Option<super::super::types::rekognition::StreamProcessorInputKinesisVideoStream>>,
}

#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamProcessorOutputS3Destination {
    /// Name of the Amazon S3 bucket you want to associate with the streaming video project.
    #[builder(into, default)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<Option<String>>,
    /// The prefix value of the location within the bucket that you want the information to be published to.
    #[builder(into, default)]
    #[serde(rename = "keyPrefix")]
    pub r#key_prefix: Box<Option<String>>,
}
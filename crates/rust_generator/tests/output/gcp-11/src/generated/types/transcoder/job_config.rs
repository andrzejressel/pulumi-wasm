#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobConfig {
    /// Ad break.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "adBreaks")]
    pub r#ad_breaks: Box<Option<Vec<super::super::types::transcoder::JobConfigAdBreak>>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "editLists")]
    pub r#edit_lists: Box<Option<Vec<super::super::types::transcoder::JobConfigEditList>>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "elementaryStreams")]
    pub r#elementary_streams: Box<Option<Vec<super::super::types::transcoder::JobConfigElementaryStream>>>,
    /// List of encryption configurations for the content.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "encryptions")]
    pub r#encryptions: Box<Option<Vec<super::super::types::transcoder::JobConfigEncryption>>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "inputs")]
    pub r#inputs: Box<Option<Vec<super::super::types::transcoder::JobConfigInput>>>,
    /// Manifest configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "manifests")]
    pub r#manifests: Box<Option<Vec<super::super::types::transcoder::JobConfigManifest>>>,
    /// Multiplexing settings for output stream.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "muxStreams")]
    pub r#mux_streams: Box<Option<Vec<super::super::types::transcoder::JobConfigMuxStream>>>,
    /// Location of output file(s) in a Cloud Storage bucket.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "output")]
    pub r#output: Box<Option<super::super::types::transcoder::JobConfigOutput>>,
    /// List of overlays on the output video, in descending Z-order.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "overlays")]
    pub r#overlays: Box<Option<Vec<super::super::types::transcoder::JobConfigOverlay>>>,
    /// Pub/Sub destination.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "pubsubDestination")]
    pub r#pubsub_destination: Box<Option<super::super::types::transcoder::JobConfigPubsubDestination>>,
}

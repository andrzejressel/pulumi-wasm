#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct JobTemplateConfig {
    /// Ad break.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "adBreaks")]
    pub r#ad_breaks: Box<Option<Vec<super::super::types::transcoder::JobTemplateConfigAdBreak>>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "editLists")]
    pub r#edit_lists: Box<Option<Vec<super::super::types::transcoder::JobTemplateConfigEditList>>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "elementaryStreams")]
    pub r#elementary_streams: Box<Option<Vec<super::super::types::transcoder::JobTemplateConfigElementaryStream>>>,
    /// List of encryption configurations for the content.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "encryptions")]
    pub r#encryptions: Box<Option<Vec<super::super::types::transcoder::JobTemplateConfigEncryption>>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "inputs")]
    pub r#inputs: Box<Option<Vec<super::super::types::transcoder::JobTemplateConfigInput>>>,
    /// Manifest configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "manifests")]
    pub r#manifests: Box<Option<Vec<super::super::types::transcoder::JobTemplateConfigManifest>>>,
    /// Multiplexing settings for output stream.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "muxStreams")]
    pub r#mux_streams: Box<Option<Vec<super::super::types::transcoder::JobTemplateConfigMuxStream>>>,
    /// Location of output file(s) in a Cloud Storage bucket.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "output")]
    pub r#output: Box<Option<super::super::types::transcoder::JobTemplateConfigOutput>>,
    /// List of overlays on the output video, in descending Z-order.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "overlays")]
    pub r#overlays: Box<Option<Vec<super::super::types::transcoder::JobTemplateConfigOverlay>>>,
    /// Pub/Sub destination.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "pubsubDestination")]
    pub r#pubsub_destination: Box<Option<super::super::types::transcoder::JobTemplateConfigPubsubDestination>>,
}

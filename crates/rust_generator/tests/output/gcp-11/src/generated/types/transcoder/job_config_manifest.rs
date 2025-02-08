#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobConfigManifest {
    /// The name of the generated file. The default is `manifest`.
    #[builder(into, default)]
    #[serde(rename = "fileName")]
    pub r#file_name: Box<Option<String>>,
    /// List of user supplied MuxStream.key values that should appear in this manifest.
    #[builder(into, default)]
    #[serde(rename = "muxStreams")]
    pub r#mux_streams: Box<Option<Vec<String>>>,
    /// Type of the manifest.
    /// Possible values are: `MANIFEST_TYPE_UNSPECIFIED`, `HLS`, `DASH`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PresetAudioCodecOptions {
    /// The bit depth of a sample is how many bits of information are included in the audio samples. Valid values are `16` and `24`. (FLAC/PCM Only)
    #[builder(into, default)]
    #[serde(rename = "bitDepth")]
    pub r#bit_depth: Box<Option<String>>,
    /// The order the bits of a PCM sample are stored in. The supported value is LittleEndian. (PCM Only)
    #[builder(into, default)]
    #[serde(rename = "bitOrder")]
    pub r#bit_order: Box<Option<String>>,
    /// If you specified AAC for Audio:Codec, choose the AAC profile for the output file.
    #[builder(into, default)]
    #[serde(rename = "profile")]
    pub r#profile: Box<Option<String>>,
    /// Whether audio samples are represented with negative and positive numbers (signed) or only positive numbers (unsigned). The supported value is Signed. (PCM Only)
    #[builder(into, default)]
    #[serde(rename = "signed")]
    pub r#signed: Box<Option<String>>,
}

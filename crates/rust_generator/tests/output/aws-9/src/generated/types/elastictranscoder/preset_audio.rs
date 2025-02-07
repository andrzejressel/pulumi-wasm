#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PresetAudio {
    /// The method of organizing audio channels and tracks. Use Audio:Channels to specify the number of channels in your output, and Audio:AudioPackingMode to specify the number of tracks and their relation to the channels. If you do not specify an Audio:AudioPackingMode, Elastic Transcoder uses SingleTrack.
    #[builder(into, default)]
    #[serde(rename = "audioPackingMode")]
    pub r#audio_packing_mode: Box<Option<String>>,
    /// The bit rate of the audio stream in the output file, in kilobits/second. Enter an integer between 64 and 320, inclusive.
    #[builder(into, default)]
    #[serde(rename = "bitRate")]
    pub r#bit_rate: Box<Option<String>>,
    /// The number of audio channels in the output file
    #[builder(into, default)]
    #[serde(rename = "channels")]
    pub r#channels: Box<Option<String>>,
    /// The audio codec for the output file. Valid values are `AAC`, `flac`, `mp2`, `mp3`, `pcm`, and `vorbis`.
    #[builder(into, default)]
    #[serde(rename = "codec")]
    pub r#codec: Box<Option<String>>,
    /// The sample rate of the audio stream in the output file, in hertz. Valid values are: `auto`, `22050`, `32000`, `44100`, `48000`, `96000`
    #[builder(into, default)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Box<Option<String>>,
}

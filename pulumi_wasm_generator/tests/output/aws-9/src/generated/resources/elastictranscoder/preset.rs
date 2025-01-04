/// Provides an Elastic Transcoder preset resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   bar:
///     type: aws:elastictranscoder:Preset
///     properties:
///       container: mp4
///       description: Sample Preset
///       name: sample_preset
///       audio:
///         audioPackingMode: SingleTrack
///         bitRate: 96
///         channels: 2
///         codec: AAC
///         sampleRate: 44100
///       audioCodecOptions:
///         profile: AAC-LC
///       video:
///         bitRate: '1600'
///         codec: H.264
///         displayAspectRatio: 16:9
///         fixedGop: 'false'
///         frameRate: auto
///         maxFrameRate: '60'
///         keyframesMaxDist: 240
///         maxHeight: auto
///         maxWidth: auto
///         paddingPolicy: Pad
///         sizingPolicy: Fit
///       videoCodecOptions:
///         Profile: main
///         Level: '2.2'
///         MaxReferenceFrames: 3
///         InterlacedMode: Progressive
///         ColorSpaceConversionMode: None
///       videoWatermarks:
///         - id: Test
///           maxWidth: 20%
///           maxHeight: 20%
///           sizingPolicy: ShrinkToFit
///           horizontalAlign: Right
///           horizontalOffset: 10px
///           verticalAlign: Bottom
///           verticalOffset: 10px
///           opacity: '55.5'
///           target: Content
///       thumbnails:
///         format: png
///         interval: 120
///         maxWidth: auto
///         maxHeight: auto
///         paddingPolicy: Pad
///         sizingPolicy: Fit
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Elastic Transcoder presets using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:elastictranscoder/preset:Preset basic_preset 1407981661351-cttk8b
/// ```
pub mod preset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PresetArgs {
        /// Audio parameters object (documented below).
        #[builder(into, default)]
        pub audio: pulumi_wasm_rust::Output<
            Option<super::super::types::elastictranscoder::PresetAudio>,
        >,
        /// Codec options for the audio parameters (documented below)
        #[builder(into, default)]
        pub audio_codec_options: pulumi_wasm_rust::Output<
            Option<super::super::types::elastictranscoder::PresetAudioCodecOptions>,
        >,
        /// The container type for the output file. Valid values are `flac`, `flv`, `fmp4`, `gif`, `mp3`, `mp4`, `mpg`, `mxf`, `oga`, `ogg`, `ts`, and `webm`.
        #[builder(into)]
        pub container: pulumi_wasm_rust::Output<String>,
        /// A description of the preset (maximum 255 characters)
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the preset. (maximum 40 characters)
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Thumbnail parameters object (documented below)
        #[builder(into, default)]
        pub thumbnails: pulumi_wasm_rust::Output<
            Option<super::super::types::elastictranscoder::PresetThumbnails>,
        >,
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// Video parameters object (documented below)
        #[builder(into, default)]
        pub video: pulumi_wasm_rust::Output<
            Option<super::super::types::elastictranscoder::PresetVideo>,
        >,
        /// Codec options for the video parameters
        #[builder(into, default)]
        pub video_codec_options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Watermark parameters for the video parameters (documented below)
        #[builder(into, default)]
        pub video_watermarks: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::elastictranscoder::PresetVideoWatermark>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PresetResult {
        /// Amazon Resource Name (ARN) of the Elastic Transcoder Preset.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Audio parameters object (documented below).
        pub audio: pulumi_wasm_rust::Output<
            Option<super::super::types::elastictranscoder::PresetAudio>,
        >,
        /// Codec options for the audio parameters (documented below)
        pub audio_codec_options: pulumi_wasm_rust::Output<
            super::super::types::elastictranscoder::PresetAudioCodecOptions,
        >,
        /// The container type for the output file. Valid values are `flac`, `flv`, `fmp4`, `gif`, `mp3`, `mp4`, `mpg`, `mxf`, `oga`, `ogg`, `ts`, and `webm`.
        pub container: pulumi_wasm_rust::Output<String>,
        /// A description of the preset (maximum 255 characters)
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the preset. (maximum 40 characters)
        pub name: pulumi_wasm_rust::Output<String>,
        /// Thumbnail parameters object (documented below)
        pub thumbnails: pulumi_wasm_rust::Output<
            Option<super::super::types::elastictranscoder::PresetThumbnails>,
        >,
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Video parameters object (documented below)
        pub video: pulumi_wasm_rust::Output<
            Option<super::super::types::elastictranscoder::PresetVideo>,
        >,
        /// Codec options for the video parameters
        pub video_codec_options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Watermark parameters for the video parameters (documented below)
        pub video_watermarks: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::elastictranscoder::PresetVideoWatermark>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PresetArgs) -> PresetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audio_binding = args.audio.get_inner();
        let audio_codec_options_binding = args.audio_codec_options.get_inner();
        let container_binding = args.container.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let thumbnails_binding = args.thumbnails.get_inner();
        let type__binding = args.type_.get_inner();
        let video_binding = args.video.get_inner();
        let video_codec_options_binding = args.video_codec_options.get_inner();
        let video_watermarks_binding = args.video_watermarks.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elastictranscoder/preset:Preset".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "audio".into(),
                    value: &audio_binding,
                },
                register_interface::ObjectField {
                    name: "audioCodecOptions".into(),
                    value: &audio_codec_options_binding,
                },
                register_interface::ObjectField {
                    name: "container".into(),
                    value: &container_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "thumbnails".into(),
                    value: &thumbnails_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "video".into(),
                    value: &video_binding,
                },
                register_interface::ObjectField {
                    name: "videoCodecOptions".into(),
                    value: &video_codec_options_binding,
                },
                register_interface::ObjectField {
                    name: "videoWatermarks".into(),
                    value: &video_watermarks_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "audio".into(),
                },
                register_interface::ResultField {
                    name: "audioCodecOptions".into(),
                },
                register_interface::ResultField {
                    name: "container".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "thumbnails".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "video".into(),
                },
                register_interface::ResultField {
                    name: "videoCodecOptions".into(),
                },
                register_interface::ResultField {
                    name: "videoWatermarks".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PresetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            audio: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("audio").unwrap(),
            ),
            audio_codec_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("audioCodecOptions").unwrap(),
            ),
            container: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("container").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            thumbnails: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thumbnails").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            video: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("video").unwrap(),
            ),
            video_codec_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("videoCodecOptions").unwrap(),
            ),
            video_watermarks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("videoWatermarks").unwrap(),
            ),
        }
    }
}

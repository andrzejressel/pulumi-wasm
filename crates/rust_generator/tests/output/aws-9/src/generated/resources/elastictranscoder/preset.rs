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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PresetArgs {
        /// Audio parameters object (documented below).
        #[builder(into, default)]
        pub audio: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::elastictranscoder::PresetAudio>,
        >,
        /// Codec options for the audio parameters (documented below)
        #[builder(into, default)]
        pub audio_codec_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::elastictranscoder::PresetAudioCodecOptions>,
        >,
        /// The container type for the output file. Valid values are `flac`, `flv`, `fmp4`, `gif`, `mp3`, `mp4`, `mpg`, `mxf`, `oga`, `ogg`, `ts`, and `webm`.
        #[builder(into)]
        pub container: pulumi_wasm_rust::InputOrOutput<String>,
        /// A description of the preset (maximum 255 characters)
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the preset. (maximum 40 characters)
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Thumbnail parameters object (documented below)
        #[builder(into, default)]
        pub thumbnails: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::elastictranscoder::PresetThumbnails>,
        >,
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Video parameters object (documented below)
        #[builder(into, default)]
        pub video: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::elastictranscoder::PresetVideo>,
        >,
        /// Codec options for the video parameters
        #[builder(into, default)]
        pub video_codec_options: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Watermark parameters for the video parameters (documented below)
        #[builder(into, default)]
        pub video_watermarks: pulumi_wasm_rust::InputOrOutput<
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PresetArgs,
    ) -> PresetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audio_binding = args.audio.get_output(context).get_inner();
        let audio_codec_options_binding = args
            .audio_codec_options
            .get_output(context)
            .get_inner();
        let container_binding = args.container.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let thumbnails_binding = args.thumbnails.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let video_binding = args.video.get_output(context).get_inner();
        let video_codec_options_binding = args
            .video_codec_options
            .get_output(context)
            .get_inner();
        let video_watermarks_binding = args
            .video_watermarks
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elastictranscoder/preset:Preset".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        PresetResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            audio: pulumi_wasm_rust::__private::into_domain(o.extract_field("audio")),
            audio_codec_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("audioCodecOptions"),
            ),
            container: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("container"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            thumbnails: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("thumbnails"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            video: pulumi_wasm_rust::__private::into_domain(o.extract_field("video")),
            video_codec_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("videoCodecOptions"),
            ),
            video_watermarks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("videoWatermarks"),
            ),
        }
    }
}

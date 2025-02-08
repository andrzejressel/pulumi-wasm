/// Transcoding Job Resource
///
///
/// To get more information about Job, see:
///
/// * [API documentation](https://cloud.google.com/transcoder/docs/reference/rest/v1/projects.locations.jobs)
/// * How-to Guides
///     * [Transcoder](https://cloud.google.com/transcoder/docs/)
///
/// ## Example Usage
///
/// ### Transcoder Job Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:storage:Bucket
///     properties:
///       name: transcoder-job
///       location: US
///       forceDestroy: true
///       uniformBucketLevelAccess: true
///       publicAccessPrevention: enforced
///   exampleMp4:
///     type: gcp:storage:BucketObject
///     name: example_mp4
///     properties:
///       name: example.mp4
///       source:
///         fn::FileAsset: ./test-fixtures/example.mp4
///       bucket: ${default.name}
///   defaultJob:
///     type: gcp:transcoder:Job
///     name: default
///     properties:
///       templateId: ${defaultJobTemplate.name}
///       location: us-central1
///       labels:
///         label: key
///   defaultJobTemplate:
///     type: gcp:transcoder:JobTemplate
///     name: default
///     properties:
///       jobTemplateId: example-job-template
///       location: us-central1
///       config:
///         inputs:
///           - key: input0
///             uri: gs://${default.name}/${exampleMp4.name}
///         output:
///           uri: gs://${default.name}/outputs/
///         editLists:
///           - key: atom0
///             inputs:
///               - input0
///             startTimeOffset: 0s
///         elementaryStreams:
///           - key: video-stream0
///             videoStream:
///               h264:
///                 widthPixels: 640
///                 heightPixels: 360
///                 bitrateBps: 550000
///                 frameRate: 60
///                 pixelFormat: yuv420p
///                 rateControlMode: vbr
///                 crfLevel: 21
///                 gopDuration: 3s
///                 vbvSizeBits: 550000
///                 vbvFullnessBits: 495000
///                 entropyCoder: cabac
///                 profile: high
///                 preset: veryfast
///           - key: video-stream1
///             videoStream:
///               h264:
///                 widthPixels: 1280
///                 heightPixels: 720
///                 bitrateBps: 550000
///                 frameRate: 60
///                 pixelFormat: yuv420p
///                 rateControlMode: vbr
///                 crfLevel: 21
///                 gopDuration: 3s
///                 vbvSizeBits: 2.5e+06
///                 vbvFullnessBits: 2.25e+06
///                 entropyCoder: cabac
///                 profile: high
///                 preset: veryfast
///           - key: audio-stream0
///             audioStream:
///               codec: aac
///               bitrateBps: 64000
///               channelCount: 2
///               channelLayouts:
///                 - fl
///                 - fr
///               sampleRateHertz: 48000
///         muxStreams:
///           - key: sd
///             fileName: sd.mp4
///             container: mp4
///             elementaryStreams:
///               - video-stream0
///               - audio-stream0
///           - key: hd
///             fileName: hd.mp4
///             container: mp4
///             elementaryStreams:
///               - video-stream1
///               - audio-stream0
///       labels:
///         label: key
/// ```
/// ### Transcoder Job Pubsub
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:storage:Bucket
///     properties:
///       name: transcoder-job
///       location: US
///       forceDestroy: true
///       uniformBucketLevelAccess: true
///       publicAccessPrevention: enforced
///   exampleMp4:
///     type: gcp:storage:BucketObject
///     name: example_mp4
///     properties:
///       name: example.mp4
///       source:
///         fn::FileAsset: ./test-fixtures/example.mp4
///       bucket: ${default.name}
///   transcoderNotifications:
///     type: gcp:pubsub:Topic
///     name: transcoder_notifications
///     properties:
///       name: transcoder-notifications
///   defaultJob:
///     type: gcp:transcoder:Job
///     name: default
///     properties:
///       location: us-central1
///       config:
///         inputs:
///           - key: input0
///             uri: gs://${default.name}/${exampleMp4.name}
///         editLists:
///           - key: atom0
///             inputs:
///               - input0
///             startTimeOffset: 0s
///         adBreaks:
///           - startTimeOffset: 3.500s
///         elementaryStreams:
///           - key: video-stream0
///             videoStream:
///               h264:
///                 widthPixels: 640
///                 heightPixels: 360
///                 bitrateBps: 550000
///                 frameRate: 60
///                 pixelFormat: yuv420p
///                 rateControlMode: vbr
///                 crfLevel: 21
///                 gopDuration: 3s
///                 vbvSizeBits: 550000
///                 vbvFullnessBits: 495000
///                 entropyCoder: cabac
///                 profile: high
///                 preset: veryfast
///           - key: video-stream1
///             videoStream:
///               h264:
///                 widthPixels: 1280
///                 heightPixels: 720
///                 bitrateBps: 550000
///                 frameRate: 60
///                 pixelFormat: yuv420p
///                 rateControlMode: vbr
///                 crfLevel: 21
///                 gopDuration: 3s
///                 vbvSizeBits: 2.5e+06
///                 vbvFullnessBits: 2.25e+06
///                 entropyCoder: cabac
///                 profile: high
///                 preset: veryfast
///           - key: audio-stream0
///             audioStream:
///               codec: aac
///               bitrateBps: 64000
///               channelCount: 2
///               channelLayouts:
///                 - fl
///                 - fr
///               sampleRateHertz: 48000
///         muxStreams:
///           - key: sd
///             fileName: sd.mp4
///             container: mp4
///             elementaryStreams:
///               - video-stream0
///               - audio-stream0
///           - key: hd
///             fileName: hd.mp4
///             container: mp4
///             elementaryStreams:
///               - video-stream1
///               - audio-stream0
///         pubsubDestination:
///           topic: ${transcoderNotifications.id}
///         output:
///           uri: gs://${default.name}/outputs/
///       labels:
///         label: key
/// ```
/// ### Transcoder Job Encryptions
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:storage:Bucket
///     properties:
///       name: transcoder-job
///       location: US
///       forceDestroy: true
///       uniformBucketLevelAccess: true
///       publicAccessPrevention: enforced
///   exampleMp4:
///     type: gcp:storage:BucketObject
///     name: example_mp4
///     properties:
///       name: example.mp4
///       source:
///         fn::FileAsset: ./test-fixtures/example.mp4
///       bucket: ${default.name}
///   encryptionKey:
///     type: gcp:secretmanager:Secret
///     name: encryption_key
///     properties:
///       secretId: transcoder-encryption-key
///       replication:
///         auto: {}
///   encryptionKeySecretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: encryption_key
///     properties:
///       secret: ${encryptionKey.name}
///       secretData: 4A67F2C1B8E93A4F6D3E7890A1BC23DF
///   # this is required to allow the transcoder service identity to access the secret
///   transcoder:
///     type: gcp:projects:ServiceIdentity
///     properties:
///       project: ${project.projectId}
///       service: transcoder.googleapis.com
///   transcoderEncryptionKeyAccessor:
///     type: gcp:secretmanager:SecretIamMember
///     name: transcoder_encryption_key_accessor
///     properties:
///       secretId: ${encryptionKey.secretId}
///       project: ${encryptionKey.project}
///       role: roles/secretmanager.secretAccessor
///       member: serviceAccount:${transcoder.email}
///   defaultJob:
///     type: gcp:transcoder:Job
///     name: default
///     properties:
///       location: us-central1
///       config:
///         inputs:
///           - key: input0
///             uri: gs://${default.name}/${exampleMp4.name}
///         elementaryStreams:
///           - key: es_video
///             videoStream:
///               h264:
///                 profile: main
///                 heightPixels: 600
///                 widthPixels: 800
///                 bitrateBps: 1e+06
///                 frameRate: 60
///           - key: es_audio
///             audioStream:
///               codec: aac
///               channelCount: 2
///               bitrateBps: 160000
///         encryptions:
///           - id: aes-128
///             secretManagerKeySource:
///               secretVersion: ${encryptionKeySecretVersion.name}
///             drmSystems:
///               clearkey: {}
///             aes128: {}
///           - id: cenc
///             secretManagerKeySource:
///               secretVersion: ${encryptionKeySecretVersion.name}
///             drmSystems:
///               widevine: {}
///             mpegCenc:
///               scheme: cenc
///           - id: cbcs
///             secretManagerKeySource:
///               secretVersion: ${encryptionKeySecretVersion.name}
///             drmSystems:
///               widevine: {}
///             mpegCenc:
///               scheme: cbcs
///         muxStreams:
///           - key: ts_aes128
///             container: ts
///             elementaryStreams:
///               - es_video
///               - es_audio
///             segmentSettings:
///               segmentDuration: 6s
///             encryptionId: aes-128
///           - key: fmp4_cenc_video
///             container: fmp4
///             elementaryStreams:
///               - es_video
///             segmentSettings:
///               segmentDuration: 6s
///             encryptionId: cenc
///           - key: fmp4_cenc_audio
///             container: fmp4
///             elementaryStreams:
///               - es_audio
///             segmentSettings:
///               segmentDuration: 6s
///             encryptionId: cenc
///           - key: fmp4_cbcs_video
///             container: fmp4
///             elementaryStreams:
///               - es_video
///             segmentSettings:
///               segmentDuration: 6s
///             encryptionId: cbcs
///           - key: fmp4_cbcs_audio
///             container: fmp4
///             elementaryStreams:
///               - es_audio
///             segmentSettings:
///               segmentDuration: 6s
///             encryptionId: cbcs
///         manifests:
///           - fileName: manifest_aes128.m3u8
///             type: HLS
///             muxStreams:
///               - ts_aes128
///           - fileName: manifest_cenc.mpd
///             type: DASH
///             muxStreams:
///               - fmp4_cenc_video
///               - fmp4_cenc_audio
///           - fileName: manifest_cbcs.mpd
///             type: DASH
///             muxStreams:
///               - fmp4_cbcs_video
///               - fmp4_cbcs_audio
///         output:
///           uri: gs://${default.name}/outputs/
///       labels:
///         label: key
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Transcoder Job Overlays
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:storage:Bucket
///     properties:
///       name: transcoder-job
///       location: US
///       forceDestroy: true
///       uniformBucketLevelAccess: true
///       publicAccessPrevention: enforced
///   exampleMp4:
///     type: gcp:storage:BucketObject
///     name: example_mp4
///     properties:
///       name: example.mp4
///       source:
///         fn::FileAsset: ./test-fixtures/example.mp4
///       bucket: ${default.name}
///   overlayPng:
///     type: gcp:storage:BucketObject
///     name: overlay_png
///     properties:
///       name: overlay.png
///       source:
///         fn::FileAsset: ./test-fixtures/overlay.png
///       bucket: ${default.name}
///   defaultJob:
///     type: gcp:transcoder:Job
///     name: default
///     properties:
///       location: us-central1
///       config:
///         inputs:
///           - key: input0
///             uri: gs://${default.name}/${exampleMp4.name}
///         editLists:
///           - key: atom0
///             inputs:
///               - input0
///             startTimeOffset: 0s
///         adBreaks:
///           - startTimeOffset: 3.500s
///         overlays:
///           - animations:
///               - animationFade:
///                   fadeType: FADE_IN
///                   startTimeOffset: 1.500s
///                   endTimeOffset: 3.500s
///                   xy:
///                     x: 1
///                     y: 0.5
///             image:
///               uri: gs://${default.name}/${overlayPng.name}
///         elementaryStreams:
///           - key: video-stream0
///             videoStream:
///               h264:
///                 widthPixels: 640
///                 heightPixels: 360
///                 bitrateBps: 550000
///                 frameRate: 60
///                 pixelFormat: yuv420p
///                 rateControlMode: vbr
///                 crfLevel: 21
///                 gopDuration: 3s
///                 vbvSizeBits: 550000
///                 vbvFullnessBits: 495000
///                 entropyCoder: cabac
///                 profile: high
///                 preset: veryfast
///           - key: video-stream1
///             videoStream:
///               h264:
///                 widthPixels: 1280
///                 heightPixels: 720
///                 bitrateBps: 550000
///                 frameRate: 60
///                 pixelFormat: yuv420p
///                 rateControlMode: vbr
///                 crfLevel: 21
///                 gopDuration: 3s
///                 vbvSizeBits: 2.5e+06
///                 vbvFullnessBits: 2.25e+06
///                 entropyCoder: cabac
///                 profile: high
///                 preset: veryfast
///           - key: audio-stream0
///             audioStream:
///               codec: aac
///               bitrateBps: 64000
///               channelCount: 2
///               channelLayouts:
///                 - fl
///                 - fr
///               sampleRateHertz: 48000
///         muxStreams:
///           - key: sd
///             fileName: sd.mp4
///             container: mp4
///             elementaryStreams:
///               - video-stream0
///               - audio-stream0
///           - key: hd
///             fileName: hd.mp4
///             container: mp4
///             elementaryStreams:
///               - video-stream1
///               - audio-stream0
///         output:
///           uri: gs://${default.name}/outputs/
///       labels:
///         label: key
/// ```
/// ### Transcoder Job Manifests
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:storage:Bucket
///     properties:
///       name: transcoder-job
///       location: US
///       forceDestroy: true
///       uniformBucketLevelAccess: true
///       publicAccessPrevention: enforced
///   exampleMp4:
///     type: gcp:storage:BucketObject
///     name: example_mp4
///     properties:
///       name: example.mp4
///       source:
///         fn::FileAsset: ./test-fixtures/example.mp4
///       bucket: ${default.name}
///   defaultJob:
///     type: gcp:transcoder:Job
///     name: default
///     properties:
///       location: us-central1
///       config:
///         inputs:
///           - key: input0
///             uri: gs://${default.name}/${exampleMp4.name}
///         editLists:
///           - key: atom0
///             startTimeOffset: 0s
///             inputs:
///               - input0
///         adBreaks:
///           - startTimeOffset: 3.500s
///         elementaryStreams:
///           - key: video-stream0
///             videoStream:
///               h264:
///                 widthPixels: 640
///                 heightPixels: 360
///                 bitrateBps: 550000
///                 frameRate: 60
///                 pixelFormat: yuv420p
///                 rateControlMode: vbr
///                 crfLevel: 21
///                 gopDuration: 3s
///                 vbvSizeBits: 550000
///                 vbvFullnessBits: 495000
///                 entropyCoder: cabac
///                 profile: high
///                 preset: veryfast
///           - key: video-stream1
///             videoStream:
///               h264:
///                 widthPixels: 1280
///                 heightPixels: 720
///                 bitrateBps: 550000
///                 frameRate: 60
///                 pixelFormat: yuv420p
///                 rateControlMode: vbr
///                 crfLevel: 21
///                 gopDuration: 3s
///                 vbvSizeBits: 2.5e+06
///                 vbvFullnessBits: 2.25e+06
///                 entropyCoder: cabac
///                 profile: high
///                 preset: veryfast
///           - key: audio-stream0
///             audioStream:
///               codec: aac
///               bitrateBps: 64000
///               channelCount: 2
///               channelLayouts:
///                 - fl
///                 - fr
///               sampleRateHertz: 48000
///         muxStreams:
///           - key: sd
///             fileName: sd.mp4
///             container: mp4
///             elementaryStreams:
///               - video-stream0
///               - audio-stream0
///           - key: hd
///             fileName: hd.mp4
///             container: mp4
///             elementaryStreams:
///               - video-stream1
///               - audio-stream0
///           - key: media-sd
///             fileName: media-sd.ts
///             container: ts
///             elementaryStreams:
///               - video-stream0
///               - audio-stream0
///           - key: media-hd
///             fileName: media-hd.ts
///             container: ts
///             elementaryStreams:
///               - video-stream1
///               - audio-stream0
///           - key: video-only-sd
///             fileName: video-only-sd.m4s
///             container: fmp4
///             elementaryStreams:
///               - video-stream0
///           - key: video-only-hd
///             fileName: video-only-hd.m4s
///             container: fmp4
///             elementaryStreams:
///               - video-stream1
///           - key: audio-only
///             fileName: audio-only.m4s
///             container: fmp4
///             elementaryStreams:
///               - audio-stream0
///         manifests:
///           - fileName: manifest.m3u8
///             type: HLS
///             muxStreams:
///               - media-sd
///               - media-hd
///           - fileName: manifest.mpd
///             type: DASH
///             muxStreams:
///               - video-only-sd
///               - video-only-hd
///               - audio-only
///         output:
///           uri: gs://${default.name}/outputs/
///       labels:
///         label: key
/// ```
///
/// ## Import
///
/// Job can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Job can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:transcoder/job:Job default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:transcoder/job:Job default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:transcoder/job:Job default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// The configuration for this template.
        /// Structure is documented below.
        #[builder(into, default)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::transcoder::JobConfig>,
        >,
        /// The labels associated with this job. You can use these to organize and group your jobs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the transcoding job resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify the templateId to use for populating Job.config.
        /// The default is preset/web-hd, which is the only supported preset.
        #[builder(into, default)]
        pub template_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// The configuration for this template.
        /// Structure is documented below.
        pub config: pulumi_gestalt_rust::Output<
            super::super::types::transcoder::JobConfig,
        >,
        /// The time the job was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The time the transcoding finished.
        pub end_time: pulumi_gestalt_rust::Output<String>,
        /// The labels associated with this job. You can use these to organize and group your jobs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the transcoding job resource.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the job.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The time the transcoding started.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// The current state of the job.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Specify the templateId to use for populating Job.config.
        /// The default is preset/web-hd, which is the only supported preset.
        pub template_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: JobArgs,
    ) -> JobResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let config_binding = args.config.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let template_id_binding = args.template_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:transcoder/job:Job".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "templateId".into(),
                    value: &template_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        JobResult {
            config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("config"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            end_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endTime"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startTime"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            template_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateId"),
            ),
        }
    }
}

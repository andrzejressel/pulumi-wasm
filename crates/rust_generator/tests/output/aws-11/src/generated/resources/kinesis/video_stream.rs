/// Provides a Kinesis Video Stream resource. Amazon Kinesis Video Streams makes it easy to securely stream video from connected devices to AWS for analytics, machine learning (ML), playback, and other processing.
///
/// For more details, see the [Amazon Kinesis Documentation](https://aws.amazon.com/documentation/kinesis/).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   default:
///     type: aws:kinesis:VideoStream
///     properties:
///       name: kinesis-video-stream
///       dataRetentionInHours: 1
///       deviceName: kinesis-video-device-name
///       mediaType: video/h264
///       tags:
///         Name: kinesis-video-stream
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Kinesis Streams using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:kinesis/videoStream:VideoStream test_stream arn:aws:kinesisvideo:us-west-2:123456789012:stream/pulumi-kinesis-test/1554978910975
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod video_stream {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VideoStreamArgs {
        /// The number of hours that you want to retain the data in the stream. Kinesis Video Streams retains the data in a data store that is associated with the stream. The default value is `0`, indicating that the stream does not persist data.
        #[builder(into, default)]
        pub data_retention_in_hours: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the device that is writing to the stream. **In the current implementation, Kinesis Video Streams does not use this name.**
        #[builder(into, default)]
        pub device_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the AWS Key Management Service (AWS KMS) key that you want Kinesis Video Streams to use to encrypt stream data. If no key ID is specified, the default, Kinesis Video-managed key (`aws/kinesisvideo`) is used.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The media type of the stream. Consumers of the stream can use this information when processing the stream. For more information about media types, see [Media Types](http://www.iana.org/assignments/media-types/media-types.xhtml). If you choose to specify the MediaType, see [Naming Requirements](https://tools.ietf.org/html/rfc6838#section-4.2) for guidelines.
        #[builder(into, default)]
        pub media_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A name to identify the stream. This is unique to the
        /// AWS account and region the Stream is created in.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VideoStreamResult {
        /// The Amazon Resource Name (ARN) specifying the Stream (same as `id`)
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A time stamp that indicates when the stream was created.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// The number of hours that you want to retain the data in the stream. Kinesis Video Streams retains the data in a data store that is associated with the stream. The default value is `0`, indicating that the stream does not persist data.
        pub data_retention_in_hours: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the device that is writing to the stream. **In the current implementation, Kinesis Video Streams does not use this name.**
        pub device_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the AWS Key Management Service (AWS KMS) key that you want Kinesis Video Streams to use to encrypt stream data. If no key ID is specified, the default, Kinesis Video-managed key (`aws/kinesisvideo`) is used.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The media type of the stream. Consumers of the stream can use this information when processing the stream. For more information about media types, see [Media Types](http://www.iana.org/assignments/media-types/media-types.xhtml). If you choose to specify the MediaType, see [Naming Requirements](https://tools.ietf.org/html/rfc6838#section-4.2) for guidelines.
        pub media_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A name to identify the stream. This is unique to the
        /// AWS account and region the Stream is created in.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The version of the stream.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VideoStreamArgs,
    ) -> VideoStreamResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_retention_in_hours_binding = args
            .data_retention_in_hours
            .get_output(context);
        let device_name_binding = args.device_name.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let media_type_binding = args.media_type.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kinesis/videoStream:VideoStream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataRetentionInHours".into(),
                    value: data_retention_in_hours_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceName".into(),
                    value: device_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mediaType".into(),
                    value: media_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VideoStreamResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            data_retention_in_hours: o.get_field("dataRetentionInHours"),
            device_name: o.get_field("deviceName"),
            kms_key_id: o.get_field("kmsKeyId"),
            media_type: o.get_field("mediaType"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version: o.get_field("version"),
        }
    }
}

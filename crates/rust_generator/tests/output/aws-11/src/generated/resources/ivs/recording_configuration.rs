/// Resource for managing an AWS IVS (Interactive Video) Recording Configuration.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = recording_configuration::create(
///         "example",
///         RecordingConfigurationArgs::builder()
///             .destination_configuration(
///                 RecordingConfigurationDestinationConfiguration::builder()
///                     .s3(
///                         RecordingConfigurationDestinationConfigurationS3::builder()
///                             .bucketName("ivs-stream-archive")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("recording_configuration-1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IVS (Interactive Video) Recording Configuration using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:ivs/recordingConfiguration:RecordingConfiguration example arn:aws:ivs:us-west-2:326937407773:recording-configuration/KAk1sHBl2L47
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod recording_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecordingConfigurationArgs {
        /// Object containing destination configuration for where recorded video will be stored.
        #[builder(into)]
        pub destination_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::ivs::RecordingConfigurationDestinationConfiguration,
        >,
        /// Recording Configuration name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If a broadcast disconnects and then reconnects within the specified interval, the multiple streams will be considered a single broadcast and merged together.
        #[builder(into, default)]
        pub recording_reconnect_window_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Object containing information to enable/disable the recording of thumbnails for a live session and modify the interval at which thumbnails are generated for the live session.
        #[builder(into, default)]
        pub thumbnail_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::ivs::RecordingConfigurationThumbnailConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct RecordingConfigurationResult {
        /// ARN of the Recording Configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Object containing destination configuration for where recorded video will be stored.
        pub destination_configuration: pulumi_gestalt_rust::Output<
            super::super::types::ivs::RecordingConfigurationDestinationConfiguration,
        >,
        /// Recording Configuration name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// If a broadcast disconnects and then reconnects within the specified interval, the multiple streams will be considered a single broadcast and merged together.
        pub recording_reconnect_window_seconds: pulumi_gestalt_rust::Output<i32>,
        /// The current state of the Recording Configuration.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Object containing information to enable/disable the recording of thumbnails for a live session and modify the interval at which thumbnails are generated for the live session.
        pub thumbnail_configuration: pulumi_gestalt_rust::Output<
            super::super::types::ivs::RecordingConfigurationThumbnailConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RecordingConfigurationArgs,
    ) -> RecordingConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_configuration_binding = args
            .destination_configuration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let recording_reconnect_window_seconds_binding = args
            .recording_reconnect_window_seconds
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let thumbnail_configuration_binding = args
            .thumbnail_configuration
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ivs/recordingConfiguration:RecordingConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationConfiguration".into(),
                    value: &destination_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recordingReconnectWindowSeconds".into(),
                    value: &recording_reconnect_window_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thumbnailConfiguration".into(),
                    value: &thumbnail_configuration_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RecordingConfigurationResult {
            arn: o.get_field("arn"),
            destination_configuration: o.get_field("destinationConfiguration"),
            name: o.get_field("name"),
            recording_reconnect_window_seconds: o
                .get_field("recordingReconnectWindowSeconds"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            thumbnail_configuration: o.get_field("thumbnailConfiguration"),
        }
    }
}

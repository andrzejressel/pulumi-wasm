/// Resource for managing an AWS IVS (Interactive Video) Recording Configuration.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod recording_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecordingConfigurationArgs {
        /// Object containing destination configuration for where recorded video will be stored.
        #[builder(into)]
        pub destination_configuration: pulumi_wasm_rust::Output<
            super::super::types::ivs::RecordingConfigurationDestinationConfiguration,
        >,
        /// Recording Configuration name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// If a broadcast disconnects and then reconnects within the specified interval, the multiple streams will be considered a single broadcast and merged together.
        #[builder(into, default)]
        pub recording_reconnect_window_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Object containing information to enable/disable the recording of thumbnails for a live session and modify the interval at which thumbnails are generated for the live session.
        #[builder(into, default)]
        pub thumbnail_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ivs::RecordingConfigurationThumbnailConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct RecordingConfigurationResult {
        /// ARN of the Recording Configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Object containing destination configuration for where recorded video will be stored.
        pub destination_configuration: pulumi_wasm_rust::Output<
            super::super::types::ivs::RecordingConfigurationDestinationConfiguration,
        >,
        /// Recording Configuration name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// If a broadcast disconnects and then reconnects within the specified interval, the multiple streams will be considered a single broadcast and merged together.
        pub recording_reconnect_window_seconds: pulumi_wasm_rust::Output<i32>,
        /// The current state of the Recording Configuration.
        pub state: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Object containing information to enable/disable the recording of thumbnails for a live session and modify the interval at which thumbnails are generated for the live session.
        pub thumbnail_configuration: pulumi_wasm_rust::Output<
            super::super::types::ivs::RecordingConfigurationThumbnailConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RecordingConfigurationArgs,
    ) -> RecordingConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_configuration_binding = args
            .destination_configuration
            .get_inner();
        let name_binding = args.name.get_inner();
        let recording_reconnect_window_seconds_binding = args
            .recording_reconnect_window_seconds
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let thumbnail_configuration_binding = args.thumbnail_configuration.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ivs/recordingConfiguration:RecordingConfiguration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationConfiguration".into(),
                    value: &destination_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recordingReconnectWindowSeconds".into(),
                    value: &recording_reconnect_window_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "thumbnailConfiguration".into(),
                    value: &thumbnail_configuration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "destinationConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recordingReconnectWindowSeconds".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "thumbnailConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RecordingConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            destination_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationConfiguration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recording_reconnect_window_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recordingReconnectWindowSeconds").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            thumbnail_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thumbnailConfiguration").unwrap(),
            ),
        }
    }
}
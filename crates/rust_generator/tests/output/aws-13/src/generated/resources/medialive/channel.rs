/// Resource for managing an AWS MediaLive Channel.
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
///     let example = channel::create(
///         "example",
///         ChannelArgs::builder()
///             .channel_class("STANDARD")
///             .destinations(
///                 vec![
///                     ChannelDestination::builder().id("destination")
///                     .settings(vec![ChannelDestinationSetting::builder()
///                     .url("s3://${main.id}/test1").build_struct(),
///                     ChannelDestinationSetting::builder().url("s3://${main2.id}/test2")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .encoder_settings(
///                 ChannelEncoderSettings::builder()
///                     .audioDescriptions(
///                         vec![
///                             ChannelEncoderSettingsAudioDescription::builder()
///                             .audioSelectorName("example audio selector")
///                             .name("audio-selector").build_struct(),
///                         ],
///                     )
///                     .outputGroups(
///                         vec![
///                             ChannelEncoderSettingsOutputGroup::builder()
///                             .outputGroupSettings(ChannelEncoderSettingsOutputGroupOutputGroupSettings::builder()
///                             .archiveGroupSettings(vec![ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSetting::builder()
///                             .destination(ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingDestination::builder()
///                             .destinationRefId("destination").build_struct())
///                             .build_struct(),]).build_struct())
///                             .outputs(vec![ChannelEncoderSettingsOutputGroupOutput::builder()
///                             .audioDescriptionNames(vec!["audio-selector",])
///                             .outputName("example-name")
///                             .outputSettings(ChannelEncoderSettingsOutputGroupOutputOutputSettings::builder()
///                             .archiveOutputSettings(ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings::builder()
///                             .containerSettings(ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettings::builder()
///                             .m2tsSettings(ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2TsSettings::builder()
///                             .audioBufferModel("ATSC").bufferModel("MULTIPLEX")
///                             .rateMode("CBR").build_struct()).build_struct())
///                             .extension("m2ts").nameModifier("_1").build_struct())
///                             .build_struct()).videoDescriptionName("example-video")
///                             .build_struct(),]).build_struct(),
///                         ],
///                     )
///                     .timecodeConfig(
///                         ChannelEncoderSettingsTimecodeConfig::builder()
///                             .source("EMBEDDED")
///                             .build_struct(),
///                     )
///                     .videoDescriptions(
///                         vec![
///                             ChannelEncoderSettingsVideoDescription::builder()
///                             .name("example-video").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .input_attachments(
///                 vec![
///                     ChannelInputAttachment::builder()
///                     .inputAttachmentName("example-input")
///                     .inputId("${exampleAwsMedialiveInput.id}").build_struct(),
///                 ],
///             )
///             .input_specification(
///                 ChannelInputSpecification::builder()
///                     .codec("AVC")
///                     .inputResolution("HD")
///                     .maximumBitrate("MAX_20_MBPS")
///                     .build_struct(),
///             )
///             .name("example-channel")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MediaLive Channel using the `channel_id`. For example:
///
/// ```sh
/// $ pulumi import aws:medialive/channel:Channel example 1234567
/// ```
pub mod channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelArgs {
        /// Specification of CDI inputs for this channel. See CDI Input Specification for more details.
        #[builder(into, default)]
        pub cdi_input_specification: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::medialive::ChannelCdiInputSpecification>,
        >,
        /// Concise argument description.
        #[builder(into)]
        pub channel_class: pulumi_wasm_rust::InputOrOutput<String>,
        /// Destinations for channel. See Destinations for more details.
        #[builder(into)]
        pub destinations: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::medialive::ChannelDestination>,
        >,
        /// Encoder settings. See Encoder Settings for more details.
        #[builder(into)]
        pub encoder_settings: pulumi_wasm_rust::InputOrOutput<
            super::super::types::medialive::ChannelEncoderSettings,
        >,
        /// Input attachments for the channel. See Input Attachments for more details.
        #[builder(into)]
        pub input_attachments: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::medialive::ChannelInputAttachment>,
        >,
        /// Specification of network and file inputs for the channel.
        #[builder(into)]
        pub input_specification: pulumi_wasm_rust::InputOrOutput<
            super::super::types::medialive::ChannelInputSpecification,
        >,
        /// The log level to write to Cloudwatch logs.
        #[builder(into, default)]
        pub log_level: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Maintenance settings for this channel. See Maintenance for more details.
        #[builder(into, default)]
        pub maintenance: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::medialive::ChannelMaintenance>,
        >,
        /// Name of the Channel.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Concise argument description.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to start/stop channel. Default: `false`
        #[builder(into, default)]
        pub start_channel: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the channel. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Settings for the VPC outputs. See VPC for more details.
        #[builder(into, default)]
        pub vpc: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::medialive::ChannelVpc>,
        >,
    }
    #[allow(dead_code)]
    pub struct ChannelResult {
        /// ARN of the Channel.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specification of CDI inputs for this channel. See CDI Input Specification for more details.
        pub cdi_input_specification: pulumi_wasm_rust::Output<
            Option<super::super::types::medialive::ChannelCdiInputSpecification>,
        >,
        /// Concise argument description.
        pub channel_class: pulumi_wasm_rust::Output<String>,
        /// ID of the Channel.
        pub channel_id: pulumi_wasm_rust::Output<String>,
        /// Destinations for channel. See Destinations for more details.
        pub destinations: pulumi_wasm_rust::Output<
            Vec<super::super::types::medialive::ChannelDestination>,
        >,
        /// Encoder settings. See Encoder Settings for more details.
        pub encoder_settings: pulumi_wasm_rust::Output<
            super::super::types::medialive::ChannelEncoderSettings,
        >,
        /// Input attachments for the channel. See Input Attachments for more details.
        pub input_attachments: pulumi_wasm_rust::Output<
            Vec<super::super::types::medialive::ChannelInputAttachment>,
        >,
        /// Specification of network and file inputs for the channel.
        pub input_specification: pulumi_wasm_rust::Output<
            super::super::types::medialive::ChannelInputSpecification,
        >,
        /// The log level to write to Cloudwatch logs.
        pub log_level: pulumi_wasm_rust::Output<String>,
        /// Maintenance settings for this channel. See Maintenance for more details.
        pub maintenance: pulumi_wasm_rust::Output<
            super::super::types::medialive::ChannelMaintenance,
        >,
        /// Name of the Channel.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Concise argument description.
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to start/stop channel. Default: `false`
        pub start_channel: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of tags to assign to the channel. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Settings for the VPC outputs. See VPC for more details.
        pub vpc: pulumi_wasm_rust::Output<
            Option<super::super::types::medialive::ChannelVpc>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ChannelArgs,
    ) -> ChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cdi_input_specification_binding = args
            .cdi_input_specification
            .get_output(context)
            .get_inner();
        let channel_class_binding = args.channel_class.get_output(context).get_inner();
        let destinations_binding = args.destinations.get_output(context).get_inner();
        let encoder_settings_binding = args
            .encoder_settings
            .get_output(context)
            .get_inner();
        let input_attachments_binding = args
            .input_attachments
            .get_output(context)
            .get_inner();
        let input_specification_binding = args
            .input_specification
            .get_output(context)
            .get_inner();
        let log_level_binding = args.log_level.get_output(context).get_inner();
        let maintenance_binding = args.maintenance.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let start_channel_binding = args.start_channel.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_binding = args.vpc.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:medialive/channel:Channel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cdiInputSpecification".into(),
                    value: &cdi_input_specification_binding,
                },
                register_interface::ObjectField {
                    name: "channelClass".into(),
                    value: &channel_class_binding,
                },
                register_interface::ObjectField {
                    name: "destinations".into(),
                    value: &destinations_binding,
                },
                register_interface::ObjectField {
                    name: "encoderSettings".into(),
                    value: &encoder_settings_binding,
                },
                register_interface::ObjectField {
                    name: "inputAttachments".into(),
                    value: &input_attachments_binding,
                },
                register_interface::ObjectField {
                    name: "inputSpecification".into(),
                    value: &input_specification_binding,
                },
                register_interface::ObjectField {
                    name: "logLevel".into(),
                    value: &log_level_binding,
                },
                register_interface::ObjectField {
                    name: "maintenance".into(),
                    value: &maintenance_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "startChannel".into(),
                    value: &start_channel_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpc".into(),
                    value: &vpc_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ChannelResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            cdi_input_specification: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cdiInputSpecification"),
            ),
            channel_class: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("channelClass"),
            ),
            channel_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("channelId"),
            ),
            destinations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinations"),
            ),
            encoder_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encoderSettings"),
            ),
            input_attachments: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inputAttachments"),
            ),
            input_specification: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inputSpecification"),
            ),
            log_level: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logLevel"),
            ),
            maintenance: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maintenance"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            start_channel: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("startChannel"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc: pulumi_wasm_rust::__private::into_domain(o.extract_field("vpc")),
        }
    }
}

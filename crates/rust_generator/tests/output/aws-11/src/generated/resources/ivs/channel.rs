/// Resource for managing an AWS IVS (Interactive Video) Channel.
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
///     let example = channel::create(
///         "example",
///         ChannelArgs::builder().name("channel-1").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IVS (Interactive Video) Channel using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:ivs/channel:Channel example arn:aws:ivs:us-west-2:326937407773:channel/0Y1lcs4U7jk5
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelArgs {
        /// If `true`, channel is private (enabled for playback authorization).
        #[builder(into, default)]
        pub authorized: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Channel latency mode. Valid values: `NORMAL`, `LOW`.
        #[builder(into, default)]
        pub latency_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Channel name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Recording configuration ARN.
        #[builder(into, default)]
        pub recording_configuration_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Channel type, which determines the allowable resolution and bitrate. Valid values: `STANDARD`, `BASIC`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ChannelResult {
        /// ARN of the Channel.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// If `true`, channel is private (enabled for playback authorization).
        pub authorized: pulumi_gestalt_rust::Output<bool>,
        /// Channel ingest endpoint, part of the definition of an ingest server, used when setting up streaming software.
        pub ingest_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Channel latency mode. Valid values: `NORMAL`, `LOW`.
        pub latency_mode: pulumi_gestalt_rust::Output<String>,
        /// Channel name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Channel playback URL.
        pub playback_url: pulumi_gestalt_rust::Output<String>,
        /// Recording configuration ARN.
        pub recording_configuration_arn: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Channel type, which determines the allowable resolution and bitrate. Valid values: `STANDARD`, `BASIC`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelArgs,
    ) -> ChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorized_binding = args.authorized.get_output(context);
        let latency_mode_binding = args.latency_mode.get_output(context);
        let name_binding = args.name.get_output(context);
        let recording_configuration_arn_binding = args
            .recording_configuration_arn
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ivs/channel:Channel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorized".into(),
                    value: authorized_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "latencyMode".into(),
                    value: latency_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recordingConfigurationArn".into(),
                    value: recording_configuration_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ChannelResult {
            arn: o.get_field("arn"),
            authorized: o.get_field("authorized"),
            ingest_endpoint: o.get_field("ingestEndpoint"),
            latency_mode: o.get_field("latencyMode"),
            name: o.get_field("name"),
            playback_url: o.get_field("playbackUrl"),
            recording_configuration_arn: o.get_field("recordingConfigurationArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
        }
    }
}

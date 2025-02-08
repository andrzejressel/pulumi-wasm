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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ChannelArgs,
    ) -> ChannelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let authorized_binding = args.authorized.get_output(context).get_inner();
        let latency_mode_binding = args.latency_mode.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let recording_configuration_arn_binding = args
            .recording_configuration_arn
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ivs/channel:Channel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorized".into(),
                    value: &authorized_binding,
                },
                register_interface::ObjectField {
                    name: "latencyMode".into(),
                    value: &latency_mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recordingConfigurationArn".into(),
                    value: &recording_configuration_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ChannelResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            authorized: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorized"),
            ),
            ingest_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingestEndpoint"),
            ),
            latency_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("latencyMode"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            playback_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("playbackUrl"),
            ),
            recording_configuration_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recordingConfigurationArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}

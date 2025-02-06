/// Provides an AWS Elemental MediaPackage Channel.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let kittens = channel::create(
///         "kittens",
///         ChannelArgs::builder()
///             .channel_id("kitten-channel")
///             .description("A channel dedicated to amusing videos of kittens.")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Media Package Channels using the channel ID. For example:
///
/// ```sh
/// $ pulumi import aws:mediapackage/channel:Channel kittens kittens-channel
/// ```
pub mod channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelArgs {
        /// A unique identifier describing the channel
        #[builder(into)]
        pub channel_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A description of the channel
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ChannelResult {
        /// The ARN of the channel
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A unique identifier describing the channel
        pub channel_id: pulumi_wasm_rust::Output<String>,
        /// A description of the channel
        pub description: pulumi_wasm_rust::Output<String>,
        /// A single item list of HLS ingest information
        pub hls_ingests: pulumi_wasm_rust::Output<
            Vec<super::super::types::mediapackage::ChannelHlsIngest>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
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
        let channel_id_binding = args.channel_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:mediapackage/channel:Channel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "channelId".into(),
                    value: &channel_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ChannelResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            channel_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("channelId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            hls_ingests: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hlsIngests"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}

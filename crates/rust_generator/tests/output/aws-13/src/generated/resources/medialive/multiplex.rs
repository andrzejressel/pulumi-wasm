/// Resource for managing an AWS MediaLive Multiplex.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:medialive:Multiplex
///     properties:
///       name: example-multiplex-changed
///       availabilityZones:
///         - ${available.names[0]}
///         - ${available.names[1]}
///       multiplexSettings:
///         transportStreamBitrate: 1e+06
///         transportStreamId: 1
///         transportStreamReservedBitrate: 1
///         maximumVideoBufferDelayMilliseconds: 1000
///       startMultiplex: true
///       tags:
///         tag1: value1
/// variables:
///   available:
///     fn::invoke:
///       function: aws:getAvailabilityZones
///       arguments:
///         state: available
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MediaLive Multiplex using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:medialive/multiplex:Multiplex example 12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod multiplex {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MultiplexArgs {
        /// A list of availability zones. You must specify exactly two.
        #[builder(into)]
        pub availability_zones: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Multiplex settings. See Multiplex Settings for more details.
        #[builder(into, default)]
        pub multiplex_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::medialive::MultiplexMultiplexSettings>,
        >,
        /// name of Multiplex.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to start the Multiplex. Defaults to `false`.
        #[builder(into, default)]
        pub start_multiplex: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the Multiplex. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MultiplexResult {
        /// ARN of the Multiplex.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A list of availability zones. You must specify exactly two.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Multiplex settings. See Multiplex Settings for more details.
        pub multiplex_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::medialive::MultiplexMultiplexSettings>,
        >,
        /// name of Multiplex.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether to start the Multiplex. Defaults to `false`.
        pub start_multiplex: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A map of tags to assign to the Multiplex. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MultiplexArgs,
    ) -> MultiplexResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let availability_zones_binding = args.availability_zones.get_output(context);
        let multiplex_settings_binding = args.multiplex_settings.get_output(context);
        let name_binding = args.name.get_output(context);
        let start_multiplex_binding = args.start_multiplex.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:medialive/multiplex:Multiplex".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZones".into(),
                    value: availability_zones_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiplexSettings".into(),
                    value: multiplex_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startMultiplex".into(),
                    value: start_multiplex_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MultiplexResult {
            arn: o.get_field("arn"),
            availability_zones: o.get_field("availabilityZones"),
            multiplex_settings: o.get_field("multiplexSettings"),
            name: o.get_field("name"),
            start_multiplex: o.get_field("startMultiplex"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}

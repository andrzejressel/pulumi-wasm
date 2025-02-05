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
pub mod multiplex {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MultiplexArgs {
        /// A list of availability zones. You must specify exactly two.
        #[builder(into)]
        pub availability_zones: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Multiplex settings. See Multiplex Settings for more details.
        #[builder(into, default)]
        pub multiplex_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::medialive::MultiplexMultiplexSettings>,
        >,
        /// name of Multiplex.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to start the Multiplex. Defaults to `false`.
        #[builder(into, default)]
        pub start_multiplex: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the Multiplex. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MultiplexResult {
        /// ARN of the Multiplex.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A list of availability zones. You must specify exactly two.
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// Multiplex settings. See Multiplex Settings for more details.
        pub multiplex_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::medialive::MultiplexMultiplexSettings>,
        >,
        /// name of Multiplex.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether to start the Multiplex. Defaults to `false`.
        pub start_multiplex: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of tags to assign to the Multiplex. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
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
        args: MultiplexArgs,
    ) -> MultiplexResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_zones_binding = args
            .availability_zones
            .get_output(context)
            .get_inner();
        let multiplex_settings_binding = args
            .multiplex_settings
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let start_multiplex_binding = args
            .start_multiplex
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:medialive/multiplex:Multiplex".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZones".into(),
                    value: &availability_zones_binding,
                },
                register_interface::ObjectField {
                    name: "multiplexSettings".into(),
                    value: &multiplex_settings_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "startMultiplex".into(),
                    value: &start_multiplex_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MultiplexResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZones"),
            ),
            multiplex_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("multiplexSettings"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            start_multiplex: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("startMultiplex"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}

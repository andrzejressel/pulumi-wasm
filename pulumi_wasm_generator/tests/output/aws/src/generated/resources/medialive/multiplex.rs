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
///       Function: aws:getAvailabilityZones
///       Arguments:
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MultiplexArgs {
        /// A list of availability zones. You must specify exactly two.
        #[builder(into)]
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// Multiplex settings. See Multiplex Settings for more details.
        #[builder(into, default)]
        pub multiplex_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::medialive::MultiplexMultiplexSettings>,
        >,
        /// name of Multiplex.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to start the Multiplex. Defaults to `false`.
        #[builder(into, default)]
        pub start_multiplex: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of tags to assign to the Multiplex. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
    pub fn create(name: &str, args: MultiplexArgs) -> MultiplexResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_zones_binding = args.availability_zones.get_inner();
        let multiplex_settings_binding = args.multiplex_settings.get_inner();
        let name_binding = args.name.get_inner();
        let start_multiplex_binding = args.start_multiplex.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:medialive/multiplex:Multiplex".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "multiplexSettings".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "startMultiplex".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MultiplexResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            multiplex_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiplexSettings").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            start_multiplex: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startMultiplex").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
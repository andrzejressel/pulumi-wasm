/// Resource for managing an AWS MediaLive MultiplexProgram.
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
///   exampleMultiplexProgram:
///     type: aws:medialive:MultiplexProgram
///     name: example
///     properties:
///       programName: example_program
///       multiplexId: ${example.id}
///       multiplexProgramSettings:
///         programNumber: 1
///         preferredChannelPipeline: CURRENTLY_ACTIVE
///         videoSettings:
///           constantBitrate: 100000
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
/// Using `pulumi import`, import MediaLive MultiplexProgram using the `id`, or a combination of "`program_name`/`multiplex_id`". For example:
///
/// ```sh
/// $ pulumi import aws:medialive/multiplexProgram:MultiplexProgram example example_program/1234567
/// ```
pub mod multiplex_program {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MultiplexProgramArgs {
        /// Multiplex ID.
        #[builder(into)]
        pub multiplex_id: pulumi_wasm_rust::Output<String>,
        /// MultiplexProgram settings. See Multiplex Program Settings for more details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub multiplex_program_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::medialive::MultiplexProgramMultiplexProgramSettings,
            >,
        >,
        /// Unique program name.
        #[builder(into)]
        pub program_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MultiplexProgramResult {
        /// Multiplex ID.
        pub multiplex_id: pulumi_wasm_rust::Output<String>,
        /// MultiplexProgram settings. See Multiplex Program Settings for more details.
        ///
        /// The following arguments are optional:
        pub multiplex_program_settings: pulumi_wasm_rust::Output<
            Option<
                super::super::types::medialive::MultiplexProgramMultiplexProgramSettings,
            >,
        >,
        /// Unique program name.
        pub program_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MultiplexProgramArgs) -> MultiplexProgramResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let multiplex_id_binding = args.multiplex_id.get_inner();
        let multiplex_program_settings_binding = args
            .multiplex_program_settings
            .get_inner();
        let program_name_binding = args.program_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:medialive/multiplexProgram:MultiplexProgram".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "multiplexId".into(),
                    value: &multiplex_id_binding,
                },
                register_interface::ObjectField {
                    name: "multiplexProgramSettings".into(),
                    value: &multiplex_program_settings_binding,
                },
                register_interface::ObjectField {
                    name: "programName".into(),
                    value: &program_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "multiplexId".into(),
                },
                register_interface::ResultField {
                    name: "multiplexProgramSettings".into(),
                },
                register_interface::ResultField {
                    name: "programName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MultiplexProgramResult {
            multiplex_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiplexId").unwrap(),
            ),
            multiplex_program_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiplexProgramSettings").unwrap(),
            ),
            program_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("programName").unwrap(),
            ),
        }
    }
}

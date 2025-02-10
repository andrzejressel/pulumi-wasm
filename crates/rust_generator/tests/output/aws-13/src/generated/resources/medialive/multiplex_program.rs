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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod multiplex_program {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MultiplexProgramArgs {
        /// Multiplex ID.
        #[builder(into)]
        pub multiplex_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// MultiplexProgram settings. See Multiplex Program Settings for more details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub multiplex_program_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::medialive::MultiplexProgramMultiplexProgramSettings,
            >,
        >,
        /// Unique program name.
        #[builder(into)]
        pub program_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MultiplexProgramResult {
        /// Multiplex ID.
        pub multiplex_id: pulumi_gestalt_rust::Output<String>,
        /// MultiplexProgram settings. See Multiplex Program Settings for more details.
        ///
        /// The following arguments are optional:
        pub multiplex_program_settings: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::medialive::MultiplexProgramMultiplexProgramSettings,
            >,
        >,
        /// Unique program name.
        pub program_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MultiplexProgramArgs,
    ) -> MultiplexProgramResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let multiplex_id_binding = args.multiplex_id.get_output(context);
        let multiplex_program_settings_binding = args
            .multiplex_program_settings
            .get_output(context);
        let program_name_binding = args.program_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:medialive/multiplexProgram:MultiplexProgram".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiplexId".into(),
                    value: multiplex_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiplexProgramSettings".into(),
                    value: multiplex_program_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "programName".into(),
                    value: program_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MultiplexProgramResult {
            multiplex_id: o.get_field("multiplexId"),
            multiplex_program_settings: o.get_field("multiplexProgramSettings"),
            program_name: o.get_field("programName"),
        }
    }
}

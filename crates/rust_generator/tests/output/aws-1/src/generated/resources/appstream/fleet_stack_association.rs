/// Manages an AppStream Fleet Stack association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = fleet::create(
///         "example",
///         FleetArgs::builder()
///             .compute_capacity(
///                 FleetComputeCapacity::builder().desiredInstances(1).build_struct(),
///             )
///             .image_name("Amazon-AppStream2-Sample-Image-03-11-2023")
///             .instance_type("stream.standard.small")
///             .name("NAME")
///             .build_struct(),
///     );
///     let exampleFleetStackAssociation = fleet_stack_association::create(
///         "exampleFleetStackAssociation",
///         FleetStackAssociationArgs::builder()
///             .fleet_name("${example.name}")
///             .stack_name("${exampleStack.name}")
///             .build_struct(),
///     );
///     let exampleStack = stack::create(
///         "exampleStack",
///         StackArgs::builder().name("STACK NAME").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppStream Stack Fleet Association using the `fleet_name` and `stack_name` separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:appstream/fleetStackAssociation:FleetStackAssociation example fleetName/stackName
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fleet_stack_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetStackAssociationArgs {
        /// Name of the fleet.
        #[builder(into)]
        pub fleet_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the stack.
        #[builder(into)]
        pub stack_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FleetStackAssociationResult {
        /// Name of the fleet.
        pub fleet_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the stack.
        pub stack_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FleetStackAssociationArgs,
    ) -> FleetStackAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let fleet_name_binding = args.fleet_name.get_output(context);
        let stack_name_binding = args.stack_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appstream/fleetStackAssociation:FleetStackAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fleetName".into(),
                    value: fleet_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stackName".into(),
                    value: stack_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FleetStackAssociationResult {
            fleet_name: o.get_field("fleetName"),
            stack_name: o.get_field("stackName"),
        }
    }
}
